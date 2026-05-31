//+------------------------------------------------------------------+
//|                                     SupportResistanceMonitor.mq5 |
//|                               Copyright 2026, Christian Benjamin |
//|                          https://www.mql5.com/en/users/lynnchris |
//+------------------------------------------------------------------+
#property copyright "Copyright 2026, Christian Benjamin"
#property link      "https://www.mql5.com/en/users/lynnchris"
#property version   "1.0"
#property strict

//--- Alert inputs
input bool   AlertPopup        = true;
input bool   AlertSound        = true;
input string SoundFile         = "alert.wav";
input bool   PushNotifications = true;

//--- Visual inputs
input color  SupportColor      = clrGreen;
input color  ResistanceColor   = clrRed;
input int    LineWidth         = 2;
input bool   ShowLabels        = true;

//--- Tolerance inputs
input double TouchTolerancePips = 0.5;
input double ApproachZonePips   = 10.0;
input int    PipMultiplier      = 1;

//--- Candlestick pattern inputs
input bool   DetectReversalPatterns = true;
input double PatternZonePips        = 5.0;
input bool   PatternHammer          = true;
input bool   PatternEngulfing       = true;
input bool   PatternStar            = true;
input bool   PatternPiercing        = true;

//--- Retest detection input
input bool   DetectRetest = true;

//--- Arrow signal inputs
input bool   PlaceBuySellArrows = true;
input bool   ArrowOnBreakout    = true;
input bool   ArrowOnReversal    = true;
input bool   ArrowOnRetest      = true;
input bool   ArrowOnPattern     = true;
input int    ArrowShiftPips     = 5;
input int    ArrowBuyCode       = 241;
input int    ArrowSellCode      = 242;
input color  ArrowBuyColor      = clrGreen;
input color  ArrowSellColor     = clrRed;
input int    ArrowCooldownSeconds = 60;
input bool   ArrowOnlyOnePerBar   = true;

#define PREFIX "SRMonitor_"

enum ELineType { TYPE_SUPPORT, TYPE_RESISTANCE };

struct SMonitoredLine
  {
   string            name;
   double            price;
   string            label;
   ELineType         type;
   bool              exists;
   int               lastSide;
   int               prevValidSide;
   int               sideBeforeTouch;
   bool              approached;
   bool              touched;
   bool              breakoutAlerted;
   bool              reversalAlerted;
   bool              patternAlerted;
   bool              retestAlerted;
   datetime          lastTouchBarTime;
   datetime          lastPatternAlertTime;
   bool              breakoutHappened;
   datetime          breakoutTime;
   int               breakoutDirection;
   datetime          lastArrowTime;
   datetime          lastArrowBarTime;
  };

SMonitoredLine g_lines[];
int            g_nextSupportId   = 1;
int            g_nextResistanceId = 1;

//+------------------------------------------------------------------+
//| Expert initialization function - set up chart events and buttons |
//+------------------------------------------------------------------+
int OnInit()
  {
   ChartSetInteger(0, CHART_EVENT_OBJECT_CREATE, true);
   ChartSetInteger(0, CHART_EVENT_OBJECT_DELETE, true);

   CreateButton("SyncSupportBtn",   "Sync Supports",   10,  20, 120, 35, clrGreen);
   CreateButton("SyncResistanceBtn","Sync Resistances",140, 20, 120, 35, clrRed);
   CreateButton("ClearAllBtn",      "Clear All Lines", 270, 20, 120, 35, clrOrange);

   return(INIT_SUCCEEDED);
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Expert deinitialization function                                 |
//+------------------------------------------------------------------+
void OnDeinit(const int reason)
  {
   ObjectDelete(0, PREFIX+"SyncSupportBtn");
   ObjectDelete(0, PREFIX+"SyncResistanceBtn");
   ObjectDelete(0, PREFIX+"ClearAllBtn");
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Create a styled button on the chart                              |
//+------------------------------------------------------------------+
void CreateButton(string btnName,string text,int x,int y,int w,int h,color bgColor)
  {
   string objName = PREFIX + btnName;

   ObjectCreate(0, objName, OBJ_BUTTON, 0, 0, 0);
   ObjectSetInteger(0, objName, OBJPROP_XDISTANCE, x);
   ObjectSetInteger(0, objName, OBJPROP_YDISTANCE, y);
   ObjectSetInteger(0, objName, OBJPROP_XSIZE,     w);
   ObjectSetInteger(0, objName, OBJPROP_YSIZE,     h);
   ObjectSetInteger(0, objName, OBJPROP_CORNER,    CORNER_LEFT_UPPER);
   ObjectSetInteger(0, objName, OBJPROP_FONTSIZE,  11);
   ObjectSetString(0,  objName, OBJPROP_FONT,      "Segoe UI");
   ObjectSetInteger(0, objName, OBJPROP_COLOR,     clrWhite);
   ObjectSetInteger(0, objName, OBJPROP_BGCOLOR,   bgColor);
   ObjectSetInteger(0, objName, OBJPROP_BORDER_COLOR, clrDarkGray);
   ObjectSetInteger(0, objName, OBJPROP_BORDER_TYPE,  BORDER_RAISED);
   ObjectSetInteger(0, objName, OBJPROP_STATE,     false);
   ObjectSetString(0,  objName, OBJPROP_TEXT,      text);
   ObjectSetInteger(0, objName, OBJPROP_SELECTABLE, false);
   ObjectSetInteger(0, objName, OBJPROP_HIDDEN,    false);
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Handle all chart events (button clicks and object deletion)      |
//+------------------------------------------------------------------+
void OnChartEvent(const int id,const long &lparam,const double &dparam,const string &sparam)
  {
   if(id == CHARTEVENT_OBJECT_CLICK)
     {
      if(sparam == PREFIX+"SyncSupportBtn")
         SyncAllLines(TYPE_SUPPORT);
      else
         if(sparam == PREFIX+"SyncResistanceBtn")
            SyncAllLines(TYPE_RESISTANCE);
         else
            if(sparam == PREFIX+"ClearAllBtn")
               ClearAllLines();
     }
   else
      if(id == CHARTEVENT_OBJECT_DELETE)
        {
         //--- Remove deleted line from monitoring array
         for(int i = 0; i < ArraySize(g_lines); i++)
           {
            if(g_lines[i].name == sparam)
              {
               g_lines[i].exists = false;
               string labelName = sparam + "_label";
               if(ObjectFind(0, labelName) >= 0)
                  ObjectDelete(0, labelName);
               break;
              }
           }

         //--- Remove non-existent lines from array
         int newSize = 0;
         for(int i = 0; i < ArraySize(g_lines); i++)
           {
            if(g_lines[i].exists)
              {
               if(i != newSize)
                  g_lines[newSize] = g_lines[i];
               newSize++;
              }
           }
         ArrayResize(g_lines, newSize);

         UpdateClearButtonState();
        }
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Synchronize all horizontal lines of the given type               |
//+------------------------------------------------------------------+
void SyncAllLines(ELineType type)
  {
   int total = ObjectsTotal(0);
   int added = 0;

   for(int i = 0; i < total; i++)
     {
      string objName = ObjectName(0, i);
      ENUM_OBJECT objType = (ENUM_OBJECT)ObjectGetInteger(0, objName, OBJPROP_TYPE);

      if(objType == OBJ_HLINE)
        {
         double price = ObjectGetDouble(0, objName, OBJPROP_PRICE, 0);
         if(price > 0 && !IsLineMonitored(objName))
           {
            AddLineToMonitor(objName, price, type);
            added++;
           }
        }
     }

   Print("Synced ", added, " ", (type==TYPE_SUPPORT?"support":"resistance"), " lines.");
   UpdateClearButtonState();
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Check if a line is already being monitored                       |
//+------------------------------------------------------------------+
bool IsLineMonitored(string lineName)
  {
   for(int i = 0; i < ArraySize(g_lines); i++)
      if(g_lines[i].name == lineName)
         return true;

   return false;
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Add a horizontal line to the monitoring system                   |
//+------------------------------------------------------------------+
void AddLineToMonitor(string lineName,double price,ELineType type)
  {
   int id = 0;
   string baseLabel;

   if(type == TYPE_SUPPORT)
     {
      baseLabel = "Sup";
      id = g_nextSupportId++;
     }
   else
     {
      baseLabel = "Res";
      id = g_nextResistanceId++;
     }

   string displayLabel = baseLabel + IntegerToString(id) + " @ " + DoubleToString(price, _Digits);

   int sz = ArraySize(g_lines);
   ArrayResize(g_lines, sz + 1);

//--- Initialize structure fields
   g_lines[sz].name              = lineName;
   g_lines[sz].price             = price;
   g_lines[sz].label             = displayLabel;
   g_lines[sz].type              = type;
   g_lines[sz].exists            = true;
   g_lines[sz].lastSide          = 0;
   g_lines[sz].prevValidSide     = 0;
   g_lines[sz].sideBeforeTouch   = 0;
   g_lines[sz].approached        = false;
   g_lines[sz].touched           = false;
   g_lines[sz].breakoutAlerted   = false;
   g_lines[sz].reversalAlerted   = false;
   g_lines[sz].patternAlerted    = false;
   g_lines[sz].retestAlerted     = false;
   g_lines[sz].lastTouchBarTime  = 0;
   g_lines[sz].lastPatternAlertTime = 0;
   g_lines[sz].breakoutHappened  = false;
   g_lines[sz].breakoutTime      = 0;
   g_lines[sz].breakoutDirection = 0;
   g_lines[sz].lastArrowTime     = 0;
   g_lines[sz].lastArrowBarTime  = 0;

//--- Apply visual settings to the line
   color lineColor = (type == TYPE_SUPPORT) ? SupportColor : ResistanceColor;
   ObjectSetInteger(0, lineName, OBJPROP_COLOR, lineColor);
   ObjectSetInteger(0, lineName, OBJPROP_WIDTH, LineWidth);
   ObjectSetInteger(0, lineName, OBJPROP_STYLE, STYLE_SOLID);
   ObjectSetInteger(0, lineName, OBJPROP_RAY_RIGHT, true);

//--- Add text label on chart
   if(ShowLabels)
     {
      string labelName = lineName + "_label";
      int    suffix    = 0;
      string temp      = labelName;

      while(ObjectFind(0, temp) >= 0)
        {
         suffix++;
         temp = labelName + "_" + IntegerToString(suffix);
        }
      labelName = temp;

      if(ObjectCreate(0, labelName, OBJ_TEXT, 0, TimeCurrent(), price))
        {
         ObjectSetString(0,  labelName, OBJPROP_TEXT,      displayLabel);
         ObjectSetInteger(0, labelName, OBJPROP_COLOR,     lineColor);
         ObjectSetInteger(0, labelName, OBJPROP_FONTSIZE,  8);
         ObjectSetString(0,  labelName, OBJPROP_FONT,      "Arial");
         ObjectSetInteger(0, labelName, OBJPROP_SELECTABLE,false);
        }
     }

   Print("Added ", (type==TYPE_SUPPORT?"Support":"Resistance"), " line: ", lineName, " at ", price);
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Remove all monitored lines and reset counters                    |
//+------------------------------------------------------------------+
void ClearAllLines()
  {
   for(int i = ArraySize(g_lines)-1; i >= 0; i--)
     {
      string name      = g_lines[i].name;
      string labelName = name + "_label";
      if(ObjectFind(0, labelName) >= 0)
         ObjectDelete(0, labelName);
     }

   ArrayResize(g_lines, 0);
   g_nextSupportId   = 1;
   g_nextResistanceId = 1;

   UpdateClearButtonState();
   Print("All monitored lines cleared.");
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Update Clear All button color based on whether lines exist       |
//+------------------------------------------------------------------+
void UpdateClearButtonState()
  {
   bool hasLines = (ArraySize(g_lines) > 0);
   color bgColor = hasLines ? clrOrange : clrGray;
   ObjectSetInteger(0, PREFIX+"ClearAllBtn", OBJPROP_BGCOLOR, bgColor);
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Check if an arrow can be placed for this line (cooldown + per-bar) |
//+------------------------------------------------------------------+
bool CanPlaceArrow(int idx,datetime barTime)
  {
   if(!PlaceBuySellArrows)
      return false;

   if(ArrowCooldownSeconds > 0 && TimeCurrent() - g_lines[idx].lastArrowTime < ArrowCooldownSeconds)
      return false;

   if(ArrowOnlyOnePerBar && g_lines[idx].lastArrowBarTime == barTime)
      return false;

   return true;
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Place arrow on chart and record timestamp                        |
//+------------------------------------------------------------------+
void PlaceArrow(int idx,datetime barTime,double price,bool isBuy)
  {
   if(!CanPlaceArrow(idx, barTime))
      return;

   DrawArrow(barTime, price, isBuy);

   g_lines[idx].lastArrowTime    = TimeCurrent();
   g_lines[idx].lastArrowBarTime = barTime;
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Main monitoring loop - detects all price interactions            |
//+------------------------------------------------------------------+
void OnTick()
  {
   double point       = SymbolInfoDouble(_Symbol, SYMBOL_POINT);
   double touchTol    = TouchTolerancePips * PipMultiplier * point;
   double approachTol = ApproachZonePips   * PipMultiplier * point;
   double patternZone = PatternZonePips    * PipMultiplier * point;

   datetime currentBarTime = iTime(_Symbol, PERIOD_CURRENT, 0);

   for(int i = 0; i < ArraySize(g_lines); i++)
     {
      if(!g_lines[i].exists)
         continue;

      double levelPrice = g_lines[i].price;
      double bid        = SymbolInfoDouble(_Symbol, SYMBOL_BID);
      double distance   = MathAbs(bid - levelPrice);

      int currentSide = (bid < levelPrice) ? 1 : ((bid > levelPrice) ? -1 : 0);

      //--- Update previous valid side when price is not exactly on the level
      if(currentSide != 0)
         g_lines[i].prevValidSide = currentSide;

      // 1. Approach (once)
      if(distance <= approachTol && !g_lines[i].approached)
        {
         SendAlert("Approaching " + g_lines[i].label, levelPrice);
         g_lines[i].approached = true;
        }

      // 2. Touch (new bar)
      if(g_lines[i].lastTouchBarTime < currentBarTime)
        {
         double barHigh = iHigh(_Symbol, PERIOD_CURRENT, 0);
         double barLow  = iLow(_Symbol,  PERIOD_CURRENT, 0);

         if(levelPrice >= barLow - touchTol && levelPrice <= barHigh + touchTol)
           {
            if(!g_lines[i].touched)
              {
               g_lines[i].sideBeforeTouch = g_lines[i].lastSide;
               SendAlert("Touch on " + g_lines[i].label, levelPrice);
               g_lines[i].touched = true;
               g_lines[i].breakoutAlerted = false;   // new touch resets breakout flag
              }
           }
         g_lines[i].lastTouchBarTime = currentBarTime;
        }

      // 3. Breakout
      int lastSide = g_lines[i].lastSide;
      if(lastSide != 0 && currentSide != 0 && currentSide != lastSide && !g_lines[i].breakoutAlerted)
        {
         SendAlert("Breakout on " + g_lines[i].label, levelPrice);
         g_lines[i].breakoutAlerted = true;
         g_lines[i].breakoutHappened = true;
         g_lines[i].breakoutTime = TimeCurrent();

         int sideBefore = (g_lines[i].prevValidSide != 0) ? g_lines[i].prevValidSide : lastSide;
         g_lines[i].breakoutDirection = (sideBefore == 1 && currentSide == -1) ? 1 : -1;

         if(ArrowOnBreakout)
           {
            bool isBuy = (g_lines[i].breakoutDirection == 1);
            PlaceArrow(i, currentBarTime, levelPrice, isBuy);
           }
        }

      // 4. Reversal (after touch)
      if(g_lines[i].touched && g_lines[i].sideBeforeTouch != 0 && !g_lines[i].reversalAlerted &&
         currentSide == g_lines[i].sideBeforeTouch && !g_lines[i].breakoutAlerted)
        {
         SendAlert("Reversal on " + g_lines[i].label, levelPrice);
         g_lines[i].reversalAlerted = true;

         if(ArrowOnReversal)
           {
            bool isBuy = (g_lines[i].sideBeforeTouch == 1);
            PlaceArrow(i, currentBarTime, levelPrice, isBuy);
           }
        }

      // 5. Retest (after breakout)
      if(DetectRetest && g_lines[i].breakoutHappened && !g_lines[i].retestAlerted &&
         distance <= touchTol)
        {
         SendAlert("Retest of " + g_lines[i].label + " after breakout", levelPrice);
         g_lines[i].retestAlerted = true;
         g_lines[i].breakoutHappened = false;

         if(ArrowOnRetest && g_lines[i].breakoutDirection != 0)
           {
            bool isBuy = (g_lines[i].breakoutDirection == 1);
            PlaceArrow(i, currentBarTime, levelPrice, isBuy);
           }
        }

      // 6. Candlestick patterns (only on completed bars)
      if(DetectReversalPatterns && !g_lines[i].patternAlerted)
        {
         bool priceInZone = (MathAbs(bid - levelPrice) <= patternZone) ||
                            (MathAbs(iClose(_Symbol, PERIOD_CURRENT, 0) - levelPrice) <= patternZone);

         if(priceInZone)
           {
            string patternName = CheckReversalPatterns();
            if(patternName != "" && g_lines[i].lastPatternAlertTime != currentBarTime)
              {
               SendAlert(StringFormat("Pattern [%s] at %s", patternName, g_lines[i].label), levelPrice);
               g_lines[i].patternAlerted = true;
               g_lines[i].lastPatternAlertTime = currentBarTime;

               if(ArrowOnPattern)
                 {
                  bool bullish = (StringFind(patternName, "Bullish") >= 0);
                  PlaceArrow(i, currentBarTime, levelPrice, bullish);
                 }
              }
           }
        }

      //--- Reset approach/pattern flags when price moves away
      if(distance > approachTol)
        {
         if(g_lines[i].approached)
            g_lines[i].approached = false;
         if(g_lines[i].patternAlerted)
            g_lines[i].patternAlerted = false;
        }

      //--- Store side for next tick
      g_lines[i].lastSide = currentSide;
     }
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Draw arrow on chart with unique name                             |
//+------------------------------------------------------------------+
void DrawArrow(datetime time,double price,bool isBuy)
  {
   string arrowName = PREFIX + "Arrow_" + IntegerToString(GetTickCount64());
   int    arrowCode = isBuy ? ArrowBuyCode : ArrowSellCode;
   color  arrowColor = isBuy ? ArrowBuyColor : ArrowSellColor;

   double point    = SymbolInfoDouble(_Symbol, SYMBOL_POINT);
   double pipValue = PipMultiplier * point;
   double arrowPrice;

   if(isBuy)
     {
      arrowPrice = iLow(_Symbol, PERIOD_CURRENT, 0) - ArrowShiftPips * pipValue;
      if(arrowPrice <= 0)
         arrowPrice = iLow(_Symbol, PERIOD_CURRENT, 0) - point;
     }
   else
     {
      arrowPrice = iHigh(_Symbol, PERIOD_CURRENT, 0) + ArrowShiftPips * pipValue;
     }

   if(ObjectCreate(0, arrowName, OBJ_ARROW, 0, time, arrowPrice))
     {
      ObjectSetInteger(0, arrowName, OBJPROP_ARROWCODE, arrowCode);
      ObjectSetInteger(0, arrowName, OBJPROP_COLOR,     arrowColor);
      ObjectSetInteger(0, arrowName, OBJPROP_WIDTH,     2);
      ObjectSetInteger(0, arrowName, OBJPROP_STYLE,     STYLE_SOLID);
      ObjectSetInteger(0, arrowName, OBJPROP_BACK,      false);
      ObjectSetInteger(0, arrowName, OBJPROP_SELECTABLE,false);

      Print("Placed ", (isBuy?"BUY":"SELL"), " arrow at ", arrowPrice);
     }
   else
      Print("Failed to draw arrow: ", GetLastError());
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Send alert via popup, sound and/or push notification             |
//+------------------------------------------------------------------+
void SendAlert(string msg,double price)
  {
   string fullMsg = StringFormat("%s at %G", msg, price);

   if(AlertPopup)
      Alert(fullMsg);
   if(AlertSound)
      PlaySound(SoundFile);
   if(PushNotifications)
      SendNotification(fullMsg);

   Print(fullMsg);
  }
//+------------------------------------------------------------------+

//+------------------------------------------------------------------+
//| Detect common candlestick reversal patterns on completed bars    |
//+------------------------------------------------------------------+
string CheckReversalPatterns()
  {
//--- Use only completed bars: shift=1 (last closed) and shift=2 (one before)
   for(int shift = 1; shift <= 3; shift++)
     {
      double open  = iOpen(_Symbol, PERIOD_CURRENT, shift);
      double high  = iHigh(_Symbol, PERIOD_CURRENT, shift);
      double low   = iLow(_Symbol,  PERIOD_CURRENT, shift);
      double close = iClose(_Symbol, PERIOD_CURRENT, shift);

      if(open == 0 || high == 0 || low == 0 || close == 0)
         continue;

      double bodySize    = MathAbs(close - open);
      double lowerShadow = (open < close) ? open - low : close - low;
      double upperShadow = (open < close) ? high - close : high - open;

      if(PatternHammer && close > open && lowerShadow >= 2 * bodySize && upperShadow <= bodySize * 0.3)
         return "Hammer (Bullish)";

      if(PatternHammer && close < open && upperShadow >= 2 * bodySize && lowerShadow <= bodySize * 0.3)
         return "Shooting Star (Bearish)";

      if(PatternEngulfing && shift >= 2)
        {
         double prevOpen  = iOpen(_Symbol, PERIOD_CURRENT, shift + 1);
         double prevClose = iClose(_Symbol, PERIOD_CURRENT, shift + 1);
         if(prevOpen == 0 || prevClose == 0)
            continue;

         if(close > open && prevClose < prevOpen && open <= prevClose && close >= prevOpen)
            return "Bullish Engulfing";

         if(close < open && prevClose > prevOpen && open >= prevClose && close <= prevOpen)
            return "Bearish Engulfing";
        }

      if(PatternStar && shift >= 3)
        {
         double prevOpen2  = iOpen(_Symbol, PERIOD_CURRENT, shift + 2);
         double prevClose2 = iClose(_Symbol, PERIOD_CURRENT, shift + 2);
         double prevOpen1  = iOpen(_Symbol, PERIOD_CURRENT, shift + 1);
         double prevClose1 = iClose(_Symbol, PERIOD_CURRENT, shift + 1);

         if(prevOpen2 == 0 || prevClose2 == 0 || prevOpen1 == 0 || prevClose1 == 0)
            continue;

         bool morningStar = (prevClose2 < prevOpen2) &&
                            (MathAbs(prevClose1 - prevOpen1) <= (iHigh(_Symbol, PERIOD_CURRENT, shift + 1) - iLow(_Symbol, PERIOD_CURRENT, shift + 1)) * 0.3) &&
                            (close > open) &&
                            (close > (prevOpen2 + prevClose2) / 2);

         if(morningStar)
            return "Morning Star (Bullish)";

         bool eveningStar = (prevClose2 > prevOpen2) &&
                            (MathAbs(prevClose1 - prevOpen1) <= (iHigh(_Symbol, PERIOD_CURRENT, shift + 1) - iLow(_Symbol, PERIOD_CURRENT, shift + 1)) * 0.3) &&
                            (close < open) &&
                            (close < (prevOpen2 + prevClose2) / 2);

         if(eveningStar)
            return "Evening Star (Bearish)";
        }

      if(PatternPiercing && shift >= 2)
        {
         double prevOpen  = iOpen(_Symbol, PERIOD_CURRENT, shift + 1);
         double prevClose = iClose(_Symbol, PERIOD_CURRENT, shift + 1);
         if(prevOpen == 0 || prevClose == 0)
            continue;

         bool piercing = (prevClose < prevOpen) && (close > open) && (open < prevClose) &&
                         (close > (prevOpen + prevClose) / 2) && (close < prevOpen);

         if(piercing)
            return "Piercing Line (Bullish)";

         bool darkCloud = (prevClose > prevOpen) && (close < open) && (open > prevClose) &&
                          (close < (prevOpen + prevClose) / 2) && (close > prevOpen);

         if(darkCloud)
            return "Dark Cloud Cover (Bearish)";
        }
     }

   return "";
  }
//+------------------------------------------------------------------+
//+------------------------------------------------------------------+
