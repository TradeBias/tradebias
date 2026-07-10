use crate::portfolio::Ledger;

#[derive(Debug, Clone)]
pub struct TearSheet {
    pub initial_balance: f64,
    pub final_balance: f64,
    pub net_pnl: f64,
    pub net_profit: f64,
    pub total_trades: usize,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub equity_curve: Vec<f64>,
}

impl TearSheet {
    /// Generate a tear sheet from an executed Ledger
    pub fn generate(ledger: &Ledger) -> Self {
        let net_pnl = ledger.current_balance - ledger.initial_balance;
        let net_profit = net_pnl;
        
        let winning_trades: Vec<_> = ledger.trades.iter().filter(|t| t.pnl > 0.0).collect();
        let losing_trades: Vec<_> = ledger.trades.iter().filter(|t| t.pnl <= 0.0).collect();
        
        let win_rate = if ledger.trades.is_empty() {
            0.0
        } else {
            winning_trades.len() as f64 / ledger.trades.len() as f64
        };
        
        let gross_profit: f64 = winning_trades.iter().map(|t| t.pnl).sum();
        let gross_loss: f64 = losing_trades.iter().map(|t| t.pnl.abs()).sum();
        
        let profit_factor = if gross_loss == 0.0 {
            f64::INFINITY // Infinite profit factor if no losses
        } else {
            gross_profit / gross_loss
        };

        // Equity Curve and Max Drawdown
        let mut equity_curve = Vec::with_capacity(ledger.trades.len() + 1);
        equity_curve.push(ledger.initial_balance);
        
        let mut peak = ledger.initial_balance;
        let mut max_drawdown = 0.0;
        let mut current_equity = ledger.initial_balance;
        
        let mut returns = Vec::with_capacity(ledger.trades.len());
        
        for trade in &ledger.trades {
            let prev_equity = current_equity;
            current_equity += trade.pnl;
            equity_curve.push(current_equity);
            
            if current_equity > peak {
                peak = current_equity;
            }
            let dd = (peak - current_equity) / peak;
            if dd > max_drawdown {
                max_drawdown = dd;
            }
            
            if prev_equity > 0.0 {
                returns.push(trade.pnl / prev_equity);
            }
        }
        
        // Pseudo-Sharpe Ratio (using trade-by-trade returns instead of daily)
        let sharpe_ratio = if returns.len() > 1 {
            let mean = returns.iter().sum::<f64>() / returns.len() as f64;
            let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (returns.len() - 1) as f64;
            let std_dev = variance.sqrt();
            if std_dev > 0.0 {
                (mean / std_dev) * (252.0f64).sqrt() // rough annualized scaling
            } else { 0.0 }
        } else { 0.0 };

        Self {
            initial_balance: ledger.initial_balance,
            final_balance: ledger.current_balance,
            net_pnl,
            net_profit,
            total_trades: ledger.total_trades(),
            win_rate,
            profit_factor,
            max_drawdown,
            sharpe_ratio,
            equity_curve,
        }
    }
}
