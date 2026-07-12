/// Pure mathematical evaluation of a sequence of trade PnLs.
#[derive(Debug, Clone, Default)]
pub struct TearSheet {
    pub initial_balance: f64,
    pub final_balance: f64,
    pub net_pnl: f64,
    pub total_trades: usize,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub equity_curve: Vec<f64>,
}

pub fn generate_tearsheet(trade_pnls: &[f64], initial_balance: f64) -> TearSheet {
    let mut total_trades = 0;
    let mut wins = 0;
    let mut gross_profit = 0.0;
    let mut gross_loss = 0.0;
    let mut returns = Vec::with_capacity(trade_pnls.len());

    let mut equity_curve = Vec::with_capacity(trade_pnls.len() + 1);
    equity_curve.push(initial_balance);
    
    let mut current_equity = initial_balance;
    let mut peak = initial_balance;
    let mut max_drawdown = 0.0;

    for &pnl in trade_pnls {
        total_trades += 1;
        if pnl > 0.0 {
            wins += 1;
            gross_profit += pnl;
        } else {
            gross_loss += pnl.abs();
        }

        let prev_equity = current_equity;
        current_equity += pnl;
        equity_curve.push(current_equity);
        
        if current_equity > peak {
            peak = current_equity;
        }
        let dd = (peak - current_equity) / peak;
        if dd > max_drawdown {
            max_drawdown = dd;
        }
        
        if prev_equity > 0.0 {
            returns.push(pnl / prev_equity);
        }
    }

    let win_rate = if total_trades > 0 {
        wins as f64 / total_trades as f64
    } else {
        0.0
    };

    let profit_factor = if gross_loss == 0.0 {
        f64::INFINITY
    } else {
        gross_profit / gross_loss
    };

    let sharpe_ratio = if returns.len() > 1 {
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (returns.len() - 1) as f64;
        let std_dev = variance.sqrt();
        if std_dev > 0.0 {
            (mean / std_dev) * (252.0f64).sqrt() // rough annualized scaling
        } else { 0.0 }
    } else { 0.0 };

    TearSheet {
        initial_balance,
        final_balance: current_equity,
        net_pnl: current_equity - initial_balance,
        total_trades,
        win_rate,
        profit_factor,
        max_drawdown,
        sharpe_ratio,
        equity_curve,
    }
}
