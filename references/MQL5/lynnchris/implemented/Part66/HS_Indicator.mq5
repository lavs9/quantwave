//+------------------------------------------------------------------+
//|                                                 HS Indicator.mq5 |
//|                                  Copyright 2026, MetaQuotes Ltd. |
//|                                             https://www.mql5.com |
//+------------------------------------------------------------------+
#property copyright "Copyright 2026, MetaQuotes Ltd."
#property link      "https://www.mql5.com/en/users/lynnchris"
#property version   "1.00"
#property indicator_chart_window
#property indicator_buffers 0
#property indicator_plots   0

//+------------------------------------------------------------------+
//| Structures                                                       |
//+------------------------------------------------------------------+
struct SwingPoint
  {
   int               barIndex;
   double            price;
   bool              isHigh;
   datetime          time;
  };

struct Pattern
  {
   int               id;
   bool              isBearish;
   int               lsIndex,headIndex,rsIndex;
   int               neck1Index,neck2Index;      // Low after LS (neck1) and low before RS (neck2)
   double            neckSlope,neckIntercept;
   double            headPrice,neckPriceAtHead,height;
   double            score;
   bool              signalGenerated;
   datetime          signalTime;
   int               signalBar;
   datetime          detectionTime;
  };

//+------------------------------------------------------------------+
//| Inputs                                                           |
//+------------------------------------------------------------------+
input int      SwingStrength       = 3;            // Swing detection strength
input double   ShoulderTolerance   = 0.02;         // Shoulder price tolerance (2%)
input double   MinPatternSizeATR   = 1.5;          // Min pattern height in ATR
input double   MaxNecklineSlopeDeg = 30.0;         // Max neckline slope (degrees) - 30° allows gentle upward/downward
input bool     AllowDescendingNeck = true;         // Allow descending necklines (rare but valid)
input int      MinTimeSymmetry     = 50;           // Min time symmetry % (0-100)
input bool     ShowNeckline        = true;
input bool     ShowBreakoutArrow   = true;
input bool     AlertOnNewPattern   = true;
input int      MinSwingDistance    = 10;
input int      MinPatternDistance  = 50;
input double   MinScoreThreshold   = 60.0;
input color    PatternFillColor    = clrYellow;
input int      PatternOpacity      = 60;

//+------------------------------------------------------------------+
//| Globals                                                          |
//+------------------------------------------------------------------+
SwingPoint    g_swings[];
Pattern       g_patterns[];
int           g_atrHandle = INVALID_HANDLE;
string        g_prefix = "HS_";
int           g_nextPatternId = 1;

//+------------------------------------------------------------------+
//| Get current ATR value                                            |
//+------------------------------------------------------------------+
double GetATR()
  {
   double atr[1];
   return (CopyBuffer(g_atrHandle,0,0,1,atr)==1) ? atr[0] : 0;
  }

//+------------------------------------------------------------------+
//| Find lowest swing point within bar range                         |
//+------------------------------------------------------------------+
int FindLowestSwing(int startBar,int endBar)
  {
   int bestIdx=-1;
   double bestPrice=DBL_MAX;
   for(int i=0; i<ArraySize(g_swings); i++)
     {
      if(g_swings[i].isHigh)
         continue;
      if(g_swings[i].barIndex>=startBar && g_swings[i].barIndex<=endBar)
         if(g_swings[i].price<bestPrice)
           {
            bestPrice=g_swings[i].price;
            bestIdx=i;
           }
     }
   return bestIdx;
  }

//+------------------------------------------------------------------+
//| Find highest swing point within bar range                        |
//+------------------------------------------------------------------+
int FindHighestSwing(int startBar,int endBar)
  {
   int bestIdx=-1;
   double bestPrice=-DBL_MAX;
   for(int i=0; i<ArraySize(g_swings); i++)
     {
      if(!g_swings[i].isHigh)
         continue;
      if(g_swings[i].barIndex>=startBar && g_swings[i].barIndex<=endBar)
         if(g_swings[i].price>bestPrice)
           {
            bestPrice=g_swings[i].price;
            bestIdx=i;
           }
     }
   return bestIdx;
  }

//+------------------------------------------------------------------+
//| Calculate neckline price at given bar index                      |
//+------------------------------------------------------------------+
double GetNecklinePrice(const Pattern &pat,int barIndex)
  {
   return pat.neckSlope*barIndex+pat.neckIntercept;
  }

//+------------------------------------------------------------------+
//| Detect swing highs/lows using specified strength                 |
//+------------------------------------------------------------------+
void DetectSwings(const datetime &time[],const double &high[],const double &low[],int totalBars)
  {
   ArrayResize(g_swings,0);
   int start=SwingStrength;
   int end=totalBars-SwingStrength-1;
   if(end<=start)
      return;
   for(int i=start; i<=end; i++)
     {
      bool isHighSwing=true,isLowSwing=true;
      for(int j=i-SwingStrength; j<=i+SwingStrength; j++)
        {
         if(high[j]>high[i])
            isHighSwing=false;
         if(low[j]<low[i])
            isLowSwing=false;
         if(!isHighSwing && !isLowSwing)
            break;
        }
      if(isHighSwing)
        {
         SwingPoint sp={i,high[i],true,time[i]};
         ArrayResize(g_swings,ArraySize(g_swings)+1);
         g_swings[ArraySize(g_swings)-1]=sp;
        }
      else
         if(isLowSwing)
           {
            SwingPoint sp={i,low[i],false,time[i]};
            ArrayResize(g_swings,ArraySize(g_swings)+1);
            g_swings[ArraySize(g_swings)-1]=sp;
           }
     }
  }

//+------------------------------------------------------------------+
//| Compute overall pattern quality score (0-100)                    |
//+------------------------------------------------------------------+
double ComputePatternScore(const Pattern &p,const double &high[],const double &low[],double atr)
  {
   double score=0.0;
   double leftPrice=(p.isBearish ? high[p.lsIndex] : low[p.lsIndex]);
   double rightPrice=(p.isBearish ? high[p.rsIndex] : low[p.rsIndex]);
   double headPrice=p.headPrice;
   double priceDiff=MathAbs(leftPrice-rightPrice)/headPrice;
   double priceSym=MathMax(0.0,1.0-priceDiff/ShoulderTolerance);
   score+=priceSym*30.0;

   if(MinTimeSymmetry>0)
     {
      int leftDist=p.headIndex-p.lsIndex;
      int rightDist=p.rsIndex-p.headIndex;
      double timeRatio=(leftDist>0 && rightDist>0) ? (double)MathMin(leftDist,rightDist)/MathMax(leftDist,rightDist) : 0;
      score+=timeRatio*(double)MinTimeSymmetry/100.0*20.0;
     }
   else
      score+=20.0;

   double slopeDeg=MathArctan(p.neckSlope)*180.0/M_PI;
   if(MathAbs(slopeDeg)<=MaxNecklineSlopeDeg)
      score+=20.0*(1.0-MathAbs(slopeDeg)/MaxNecklineSlopeDeg);

   double sizeRatio=p.height/atr;
   double sizeScore=MathMin(30.0,(sizeRatio/MinPatternSizeATR)*30.0);
   score+=sizeScore;
   return MathMin(100.0,score);
  }

//+------------------------------------------------------------------+
//| Detect Head & Shoulders / Inverse patterns from swings           |
//+------------------------------------------------------------------+
void DetectPatterns(const datetime &time[],const double &high[],const double &low[],const double &close[],int totalBars)
  {
   if(ArraySize(g_swings)<5)
      return;
   double atr=GetATR();
   if(atr<=0)
      return;

   Pattern candidates[];
   ArrayResize(candidates,0);

   for(int i=0; i<ArraySize(g_swings)-4; i++)
     {
      //-- Bearish: High, Low, High, Low, High
      if(g_swings[i].isHigh && !g_swings[i+1].isHigh && g_swings[i+2].isHigh && !g_swings[i+3].isHigh && g_swings[i+4].isHigh)
        {
         int ls=i,n1=i+1,head=i+2,n2=i+3,rs=i+4;
         if(g_swings[rs].barIndex-g_swings[ls].barIndex<MinSwingDistance)
            continue;
         if(g_swings[head].price<=g_swings[ls].price)
            continue;
         if(g_swings[rs].price>=g_swings[head].price)
            continue;
         double shoulderDiff=MathAbs(g_swings[ls].price-g_swings[rs].price)/g_swings[head].price;
         if(shoulderDiff>ShoulderTolerance)
            continue;

         //-- Neckline connects low after LS (n1) and low before RS (n2)
         double x1=(double)g_swings[n1].barIndex,y1=g_swings[n1].price;
         double x2=(double)g_swings[n2].barIndex,y2=g_swings[n2].price;
         double slope=(y2-y1)/(x2-x1);
         double intercept=y1-slope*x1;
         double neckAtHead=slope*g_swings[head].barIndex+intercept;
         double height=g_swings[head].price-neckAtHead;
         if(height<MinPatternSizeATR*atr)
            continue;

         //-- Filter descending necklines if not allowed
         if(!AllowDescendingNeck && slope<0)
            continue;

         Pattern pat;
         pat.id=g_nextPatternId++;
         pat.isBearish=true;
         pat.lsIndex=g_swings[ls].barIndex;
         pat.headIndex=g_swings[head].barIndex;
         pat.rsIndex=g_swings[rs].barIndex;
         pat.neck1Index=g_swings[n1].barIndex;
         pat.neck2Index=g_swings[n2].barIndex;
         pat.neckSlope=slope;
         pat.neckIntercept=intercept;
         pat.headPrice=g_swings[head].price;
         pat.neckPriceAtHead=neckAtHead;
         pat.height=height;
         pat.signalGenerated=false;
         pat.detectionTime=time[0];
         pat.score=ComputePatternScore(pat,high,low,atr);
         if(pat.score>=MinScoreThreshold)
           {
            ArrayResize(candidates,ArraySize(candidates)+1);
            candidates[ArraySize(candidates)-1]=pat;
           }
        }
      //-- Bullish Inverse: Low, High, Low, High, Low
      else
         if(!g_swings[i].isHigh && g_swings[i+1].isHigh && !g_swings[i+2].isHigh && g_swings[i+3].isHigh && !g_swings[i+4].isHigh)
           {
            int ls=i,n1=i+1,head=i+2,n2=i+3,rs=i+4;
            if(g_swings[rs].barIndex-g_swings[ls].barIndex<MinSwingDistance)
               continue;
            if(g_swings[head].price>=g_swings[ls].price)
               continue;
            if(g_swings[rs].price<=g_swings[head].price)
               continue;
            double shoulderDiff=MathAbs(g_swings[ls].price-g_swings[rs].price)/MathAbs(g_swings[head].price);
            if(shoulderDiff>ShoulderTolerance)
               continue;

            //-- For bullish, neckline connects high after LS and high before RS
            double x1=(double)g_swings[n1].barIndex,y1=g_swings[n1].price;
            double x2=(double)g_swings[n2].barIndex,y2=g_swings[n2].price;
            double slope=(y2-y1)/(x2-x1);
            double intercept=y1-slope*x1;
            double neckAtHead=slope*g_swings[head].barIndex+intercept;
            double height=neckAtHead-g_swings[head].price;
            if(height<MinPatternSizeATR*atr)
               continue;

            if(!AllowDescendingNeck && slope<0)
               continue;

            Pattern pat;
            pat.id=g_nextPatternId++;
            pat.isBearish=false;
            pat.lsIndex=g_swings[ls].barIndex;
            pat.headIndex=g_swings[head].barIndex;
            pat.rsIndex=g_swings[rs].barIndex;
            pat.neck1Index=g_swings[n1].barIndex;
            pat.neck2Index=g_swings[n2].barIndex;
            pat.neckSlope=slope;
            pat.neckIntercept=intercept;
            pat.headPrice=g_swings[head].price;
            pat.neckPriceAtHead=neckAtHead;
            pat.height=height;
            pat.signalGenerated=false;
            pat.detectionTime=time[0];
            pat.score=ComputePatternScore(pat,high,low,atr);
            if(pat.score>=MinScoreThreshold)
              {
               ArrayResize(candidates,ArraySize(candidates)+1);
               candidates[ArraySize(candidates)-1]=pat;
              }
           }
     }

   //-- Deduplication and merging (same as before)
   for(int i=0; i<ArraySize(candidates)-1; i++)
      for(int j=i+1; j<ArraySize(candidates); j++)
         if(MathAbs(candidates[i].headIndex-candidates[j].headIndex)<MinPatternDistance)
            if(candidates[i].score<candidates[j].score)
               candidates[i].score=-1;
            else
               candidates[j].score=-1;
   int newSize=0;
   for(int i=0; i<ArraySize(candidates); i++)
      if(candidates[i].score>=MinScoreThreshold)
        {
         if(i!=newSize)
            candidates[newSize]=candidates[i];
         newSize++;
        }
   ArrayResize(candidates,newSize);

   int existingCount=ArraySize(g_patterns);
   for(int i=0; i<ArraySize(candidates); i++)
     {
      bool exists=false;
      for(int j=0; j<existingCount; j++)
        {
         if(g_patterns[j].lsIndex==candidates[i].lsIndex &&
            g_patterns[j].headIndex==candidates[i].headIndex &&
            g_patterns[j].rsIndex==candidates[i].rsIndex)
           {
            exists=true;
            break;
           }
        }
      if(!exists)
        {
         ArrayResize(g_patterns,existingCount+1);
         g_patterns[existingCount]=candidates[i];
         existingCount++;
         DrawPatternTriangles(candidates[i],time,high,low);
         DrawNeckline(candidates[i],time);
         if(AlertOnNewPattern)
            Alert("New ",(candidates[i].isBearish ? "Bearish" : "Bullish")," pattern on ",_Symbol);
        }
     }
  }

//+------------------------------------------------------------------+
//| Draw filled triangles for pattern visualization                  |
//+------------------------------------------------------------------+
void DrawPatternTriangles(const Pattern &pat,const datetime &time[],const double &high[],const double &low[])
  {
   string base=g_prefix+"TRI_"+IntegerToString(pat.id);
   uint argbColor=ColorToARGB(PatternFillColor,(uchar)PatternOpacity);

   int neck0Bar=-1,neck3Bar=-1;
   if(pat.isBearish)
     {
      for(int i=0; i<ArraySize(g_swings); i++)
        {
         if(g_swings[i].isHigh)
            continue;
         if(g_swings[i].barIndex<pat.lsIndex && (neck0Bar==-1 || g_swings[i].barIndex>neck0Bar))
            neck0Bar=g_swings[i].barIndex;
         if(g_swings[i].barIndex>pat.rsIndex && (neck3Bar==-1 || g_swings[i].barIndex<neck3Bar))
            neck3Bar=g_swings[i].barIndex;
        }
     }
   else
     {
      for(int i=0; i<ArraySize(g_swings); i++)
        {
         if(!g_swings[i].isHigh)
            continue;
         if(g_swings[i].barIndex<pat.lsIndex && (neck0Bar==-1 || g_swings[i].barIndex>neck0Bar))
            neck0Bar=g_swings[i].barIndex;
         if(g_swings[i].barIndex>pat.rsIndex && (neck3Bar==-1 || g_swings[i].barIndex<neck3Bar))
            neck3Bar=g_swings[i].barIndex;
        }
     }
   if(neck0Bar==-1)
      neck0Bar=pat.neck1Index;
   if(neck3Bar==-1)
      neck3Bar=pat.neck2Index;

   double neckPrice0=GetNecklinePrice(pat,neck0Bar);
   double neckPrice1=GetNecklinePrice(pat,pat.neck1Index);
   double neckPrice2=GetNecklinePrice(pat,pat.neck2Index);
   double neckPrice3=GetNecklinePrice(pat,neck3Bar);

   ObjectCreate(0,base+"_LS",OBJ_TRIANGLE,0,time[neck0Bar],neckPrice0,time[pat.lsIndex],pat.isBearish ? high[pat.lsIndex] : low[pat.lsIndex],time[pat.neck1Index],neckPrice1);
   ObjectSetInteger(0,base+"_LS",OBJPROP_COLOR,PatternFillColor);
   ObjectSetInteger(0,base+"_LS",OBJPROP_FILL,true);
   ObjectSetInteger(0,base+"_LS",OBJPROP_BACK,true);
   ObjectSetInteger(0,base+"_LS",OBJPROP_BGCOLOR,argbColor);
   ObjectCreate(0,base+"_HD",OBJ_TRIANGLE,0,time[pat.neck1Index],neckPrice1,time[pat.headIndex],pat.isBearish ? high[pat.headIndex] : low[pat.headIndex],time[pat.neck2Index],neckPrice2);
   ObjectSetInteger(0,base+"_HD",OBJPROP_COLOR,PatternFillColor);
   ObjectSetInteger(0,base+"_HD",OBJPROP_FILL,true);
   ObjectSetInteger(0,base+"_HD",OBJPROP_BACK,true);
   ObjectSetInteger(0,base+"_HD",OBJPROP_BGCOLOR,argbColor);
   ObjectCreate(0,base+"_RS",OBJ_TRIANGLE,0,time[pat.neck2Index],neckPrice2,time[pat.rsIndex],pat.isBearish ? high[pat.rsIndex] : low[pat.rsIndex],time[neck3Bar],neckPrice3);
   ObjectSetInteger(0,base+"_RS",OBJPROP_COLOR,PatternFillColor);
   ObjectSetInteger(0,base+"_RS",OBJPROP_FILL,true);
   ObjectSetInteger(0,base+"_RS",OBJPROP_BACK,true);
   ObjectSetInteger(0,base+"_RS",OBJPROP_BGCOLOR,argbColor);
  }

//+------------------------------------------------------------------+
//| Draw extended neckline trend line and pattern label              |
//+------------------------------------------------------------------+
void DrawNeckline(const Pattern &pat,const datetime &time[])
  {
   if(!ShowNeckline)
      return;
   string neckObj=g_prefix+"NECK_"+IntegerToString(pat.id);
   string labelObj=g_prefix+"LBL_"+IntegerToString(pat.id);

   //-- Find leftmost and rightmost neck points (extend the neckline)
   int neck0Bar=-1,neck3Bar=-1;
   if(pat.isBearish)
     {
      for(int i=0; i<ArraySize(g_swings); i++)
        {
         if(g_swings[i].isHigh)
            continue;
         if(g_swings[i].barIndex<pat.lsIndex && (neck0Bar==-1 || g_swings[i].barIndex>neck0Bar))
            neck0Bar=g_swings[i].barIndex;
         if(g_swings[i].barIndex>pat.rsIndex && (neck3Bar==-1 || g_swings[i].barIndex<neck3Bar))
            neck3Bar=g_swings[i].barIndex;
        }
     }
   else
     {
      for(int i=0; i<ArraySize(g_swings); i++)
        {
         if(!g_swings[i].isHigh)
            continue;
         if(g_swings[i].barIndex<pat.lsIndex && (neck0Bar==-1 || g_swings[i].barIndex>neck0Bar))
            neck0Bar=g_swings[i].barIndex;
         if(g_swings[i].barIndex>pat.rsIndex && (neck3Bar==-1 || g_swings[i].barIndex<neck3Bar))
            neck3Bar=g_swings[i].barIndex;
        }
     }
   if(neck0Bar==-1)
      neck0Bar=pat.neck1Index;
   if(neck3Bar==-1)
      neck3Bar=pat.neck2Index;

   double neckPrice0=GetNecklinePrice(pat,neck0Bar);
   double neckPrice3=GetNecklinePrice(pat,neck3Bar);

   //-- Draw or update the neckline trendline
   if(ObjectFind(0,neckObj)<0)
      ObjectCreate(0,neckObj,OBJ_TREND,0,time[neck0Bar],neckPrice0,time[neck3Bar],neckPrice3);
   else
     {
      ObjectSetInteger(0,neckObj,OBJPROP_TIME,0,time[neck0Bar]);
      ObjectSetDouble(0,neckObj,OBJPROP_PRICE,0,neckPrice0);
      ObjectSetInteger(0,neckObj,OBJPROP_TIME,1,time[neck3Bar]);
      ObjectSetDouble(0,neckObj,OBJPROP_PRICE,1,neckPrice3);
     }
   ObjectSetInteger(0,neckObj,OBJPROP_COLOR,clrMagenta);
   ObjectSetInteger(0,neckObj,OBJPROP_STYLE,STYLE_DASH);
   ObjectSetInteger(0,neckObj,OBJPROP_WIDTH,1);
   ObjectSetInteger(0,neckObj,OBJPROP_BACK,false);

   //-- Optional label (no score) – can be turned off via separate input if needed
   datetime labelTime=time[neck3Bar];
   double labelPrice=neckPrice3;
   double offset=pat.height*0.08;
   if(pat.isBearish)
      labelPrice=neckPrice3-offset;
   else
      labelPrice=neckPrice3+offset;

   string labelText=pat.isBearish ? "Head & Shoulders" : "Inverse Head & Shoulders";
   color textColor=pat.isBearish ? clrRed : clrLime;

   if(ObjectFind(0,labelObj)<0)
      ObjectCreate(0,labelObj,OBJ_TEXT,0,labelTime,labelPrice);
   else
     {
      ObjectSetInteger(0,labelObj,OBJPROP_TIME,0,labelTime);
      ObjectSetDouble(0,labelObj,OBJPROP_PRICE,0,labelPrice);
     }
   ObjectSetString(0,labelObj,OBJPROP_TEXT,labelText);
   ObjectSetInteger(0,labelObj,OBJPROP_COLOR,textColor);
   ObjectSetInteger(0,labelObj,OBJPROP_FONTSIZE,9);
   ObjectSetInteger(0,labelObj,OBJPROP_BACK,false);
   ObjectSetInteger(0,labelObj,OBJPROP_BGCOLOR,clrWhite);
   ObjectSetInteger(0,labelObj,OBJPROP_BACK,true);
   ObjectSetInteger(0,labelObj,OBJPROP_BORDER_COLOR,clrGray);
   ObjectSetInteger(0,labelObj,OBJPROP_BORDER_TYPE,BORDER_FLAT);
  }

//+------------------------------------------------------------------+
//| Draw breakout arrow signal on chart                              |
//+------------------------------------------------------------------+
void DrawBreakoutArrow(const Pattern &pat,const datetime &time[],int bar,double price)
  {
   if(!ShowBreakoutArrow)
      return;
   string base=g_prefix+"SIG_"+IntegerToString(pat.id);
   if(pat.isBearish)
     {
      ObjectCreate(0,base+"_ARR",OBJ_ARROW_DOWN,0,time[bar],price);
      ObjectSetInteger(0,base+"_ARR",OBJPROP_COLOR,clrRed);
      ObjectSetInteger(0,base+"_ARR",OBJPROP_WIDTH,2);
     }
   else
     {
      ObjectCreate(0,base+"_ARR",OBJ_ARROW_UP,0,time[bar],price);
      ObjectSetInteger(0,base+"_ARR",OBJPROP_COLOR,clrLime);
      ObjectSetInteger(0,base+"_ARR",OBJPROP_WIDTH,2);
     }
  }

//+------------------------------------------------------------------+
//| Check for neckline breakouts on each new bar                     |
//+------------------------------------------------------------------+
void CheckBreakouts(const datetime &time[],const double &close[],int currentBar)
  {
   for(int i=0; i<ArraySize(g_patterns); i++)
     {
      if(g_patterns[i].signalGenerated)
         continue;
      double neckCurrent=GetNecklinePrice(g_patterns[i],currentBar);
      bool breakout=(g_patterns[i].isBearish) ? (close[currentBar]<neckCurrent) : (close[currentBar]>neckCurrent);
      if(breakout)
        {
         g_patterns[i].signalGenerated=true;
         g_patterns[i].signalTime=time[currentBar];
         g_patterns[i].signalBar=currentBar;
         DrawBreakoutArrow(g_patterns[i],time,currentBar,close[currentBar]);
         if(AlertOnNewPattern)
            Alert("Breakout on ",(g_patterns[i].isBearish ? "Bearish" : "Bullish")," pattern at ",close[currentBar]);
        }
     }
  }

//+------------------------------------------------------------------+
//| Main OnCalculate function                                        |
//+------------------------------------------------------------------+
int OnCalculate(const int rates_total,
                const int prev_calculated,
                const datetime &time[],
                const double &open[],
                const double &high[],
                const double &low[],
                const double &close[],
                const long &tick_volume[],
                const long &volume[],
                const int &spread[])
  {
   static datetime lastTime=0;
   if(time[0]==lastTime)
      return rates_total;
   lastTime=time[0];
   int bars=rates_total;
   DetectSwings(time,high,low,bars);
   DetectPatterns(time,high,low,close,bars);
   CheckBreakouts(time,close,0);
   return rates_total;
  }

//+------------------------------------------------------------------+
//| Initialization                                                   |
//+------------------------------------------------------------------+
int OnInit()
  {
   g_atrHandle=iATR(_Symbol,PERIOD_CURRENT,14);
   if(g_atrHandle==INVALID_HANDLE)
      return INIT_FAILED;
   ObjectsDeleteAll(0,g_prefix);
   return INIT_SUCCEEDED;
  }

//+------------------------------------------------------------------+
//| Deinitialization                                                 |
//+------------------------------------------------------------------+
void OnDeinit(const int reason)
  {
   if(g_atrHandle!=INVALID_HANDLE)
      IndicatorRelease(g_atrHandle);
   ObjectsDeleteAll(0,g_prefix);
   Comment("");
  }
//+------------------------------------------------------------------+