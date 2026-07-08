from __future__ import annotations

from dataclasses import dataclass

@dataclass(frozen=True)
class PerformanceMetrics:
    """Performance metrics from a backtest.
    
    All return-like fields (total_return, cagr, max_drawdown_pct) are expressed
    as fractions, not percentages (e.g., 0.05 = 5%).
    `max_drawdown_pct` is always a positive fraction.
    """
    total_return: float
    cagr: float
    sharpe_ratio: float
    sortino_ratio: float
    max_drawdown_pct: float
    win_rate: float
    profit_factor: float
    num_trades: int
    avg_trade_pnl: float
    final_equity: float

    def as_dict(self) -> dict[str, float]:
        """Convert to dictionary for backwards compatibility."""
        from dataclasses import asdict
        return asdict(self)
        
    def keys(self):
        return self.as_dict().keys()
        
    def __getitem__(self, key: str):
        return getattr(self, key)

@dataclass(frozen=True)
class BacktestStats:
    """Core summary statistics from a backtest run."""
    initial_cash: float
    final_equity: float
    net_pnl: float
    num_trades: int
    total_return: float
    num_symbols: int | None = None
    portfolio_mode: str | None = None
    
    def as_dict(self) -> dict[str, float]:
        """Convert to dictionary for backwards compatibility."""
        from dataclasses import asdict
        return asdict(self)
        
    def keys(self):
        return self.as_dict().keys()
        
    def __getitem__(self, key: str):
        return getattr(self, key)
