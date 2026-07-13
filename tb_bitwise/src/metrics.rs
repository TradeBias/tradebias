pub struct StrategyMetrics {
    pub total_trades: u32,
    pub winning_trades: u32,
    pub total_pnl: f64,
    pub max_equity: f64,
    pub max_drawdown: f64,
    pub gross_profit: f64,
    pub gross_loss: f64,
    pub sum_ret2: f64,
    pub sum_downside_ret2: f64,
    pub sum_x: f64,
    pub sum_y: f64,
    pub sum_xy: f64,
    pub sum_x2: f64,
    pub sum_y2: f64,
    pub sum_win_ret2: f64,
    pub sum_loss_ret2: f64,
    pub max_consecutive_losses: u32,
    pub current_losing_streak: u32,
    pub largest_win: f64,
    pub largest_loss: f64,
}

#[derive(Debug, Clone)]
pub struct StrategyResult {
    pub total_trades: u32,
    pub winning_trades: u32,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub max_drawdown: f64,
    pub pnl_over_dd: f64,
    pub ratio_wl: f64,
    pub profit_factor: f64,
    pub sharpe: f64,
    pub sortino: f64,
    pub avg_trade: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub std_win: f64,
    pub std_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub max_consecutive_losses: u32,
    pub exposure_pct: f64,
    pub cpc_index: f64,
    pub corr_coef: f64,
}

impl StrategyMetrics {
    pub fn new() -> Self {
        Self {
            total_trades: 0,
            winning_trades: 0,
            total_pnl: 0.0,
            max_equity: 0.0,
            max_drawdown: 0.0,
            gross_profit: 0.0,
            gross_loss: 0.0,
            sum_ret2: 0.0,
            sum_downside_ret2: 0.0,
            sum_x: 0.0,
            sum_y: 0.0,
            sum_xy: 0.0,
            sum_x2: 0.0,
            sum_y2: 0.0,
            sum_win_ret2: 0.0,
            sum_loss_ret2: 0.0,
            max_consecutive_losses: 0,
            current_losing_streak: 0,
            largest_win: 0.0,
            largest_loss: 0.0,
        }
    }

    #[inline]
    pub fn add_trade(&mut self, trade_pnl: f64) {
        self.total_trades += 1;
        self.total_pnl += trade_pnl;

        if trade_pnl > 0.0 {
            self.winning_trades += 1;
            self.gross_profit += trade_pnl;
            self.sum_win_ret2 += trade_pnl * trade_pnl;
            self.current_losing_streak = 0;
            if trade_pnl > self.largest_win {
                self.largest_win = trade_pnl;
            }
        } else {
            self.gross_loss += trade_pnl.abs();
            self.sum_downside_ret2 += trade_pnl * trade_pnl;
            self.sum_loss_ret2 += trade_pnl * trade_pnl;
            self.current_losing_streak += 1;
            if self.current_losing_streak > self.max_consecutive_losses {
                self.max_consecutive_losses = self.current_losing_streak;
            }
            if trade_pnl < self.largest_loss { // largest_loss will be a negative number, e.g. -50.0
                self.largest_loss = trade_pnl;
            }
        }

        self.sum_ret2 += trade_pnl * trade_pnl;

        // Drawdown Tracking
        if self.total_pnl > self.max_equity {
            self.max_equity = self.total_pnl;
        }
        let current_dd = self.max_equity - self.total_pnl;
        if current_dd > self.max_drawdown {
            self.max_drawdown = current_dd;
        }

        // Correlation Tracking (Equity Curve straightness)
        let x = self.total_trades as f64;
        let y = self.total_pnl;
        self.sum_x += x;
        self.sum_y += y;
        self.sum_xy += x * y;
        self.sum_x2 += x * x;
        self.sum_y2 += y * y;
    }

    pub fn finalize(self, total_bars: usize) -> StrategyResult {
        if self.total_trades == 0 {
            return StrategyResult {
                total_trades: 0, winning_trades: 0, win_rate: 0.0, total_pnl: 0.0,
                max_drawdown: 0.0, pnl_over_dd: 0.0, ratio_wl: 0.0, profit_factor: 0.0,
                sharpe: 0.0, sortino: 0.0, avg_trade: 0.0, avg_win: 0.0, avg_loss: 0.0,
                std_win: 0.0, std_loss: 0.0, largest_win: 0.0, largest_loss: 0.0,
                max_consecutive_losses: 0, exposure_pct: 0.0, cpc_index: 0.0,
                corr_coef: 0.0,
            };
        }

        let n = self.total_trades as f64;
        let win_rate = self.winning_trades as f64 / n;
        let avg_trade = self.total_pnl / n;
        
        let pnl_over_dd = if self.max_drawdown > 0.0 { self.total_pnl / self.max_drawdown } else { self.total_pnl };
        let profit_factor = if self.gross_loss > 0.0 { self.gross_profit / self.gross_loss } else { self.gross_profit };
        
        let losing_trades = self.total_trades - self.winning_trades;
        let avg_win = if self.winning_trades > 0 { self.gross_profit / self.winning_trades as f64 } else { 0.0 };
        let avg_loss = if losing_trades > 0 { self.gross_loss / losing_trades as f64 } else { 0.0 };
        let ratio_wl = if avg_loss > 0.0 { avg_win / avg_loss } else { avg_win };

        let cpc_index = win_rate * ratio_wl * profit_factor;

        let var = (self.sum_ret2 / n) - (avg_trade * avg_trade);
        let std_dev = if var > 0.0 { var.sqrt() } else { 0.0 };
        let sharpe = if std_dev > 0.0 { avg_trade / std_dev } else { 0.0 };

        let downside_var = self.sum_downside_ret2 / n;
        let downside_dev = if downside_var > 0.0 { downside_var.sqrt() } else { 0.0 };
        let sortino = if downside_dev > 0.0 { avg_trade / downside_dev } else { 0.0 };

        // Standard deviations of winning and losing trades
        let win_n = self.winning_trades as f64;
        let win_var = if win_n > 0.0 { (self.sum_win_ret2 / win_n) - (avg_win * avg_win) } else { 0.0 };
        let std_win = if win_var > 0.0 { win_var.sqrt() } else { 0.0 };

        let loss_n = losing_trades as f64;
        let loss_var = if loss_n > 0.0 { (self.sum_loss_ret2 / loss_n) - (avg_loss * avg_loss) } else { 0.0 };
        let std_loss = if loss_var > 0.0 { loss_var.sqrt() } else { 0.0 };

        // Exposure %
        let exposure_pct = if total_bars > 0 { (self.total_trades as f64 / total_bars as f64) * 100.0 } else { 0.0 };

        // Correlation Coefficient (Pearson R) for Equity Curve Straightness
        let numerator = (n * self.sum_xy) - (self.sum_x * self.sum_y);
        let denominator = ((n * self.sum_x2 - (self.sum_x * self.sum_x)) * (n * self.sum_y2 - (self.sum_y * self.sum_y))).sqrt();
        let corr_coef = if denominator > 0.0 { numerator / denominator } else { 0.0 };

        StrategyResult {
            total_trades: self.total_trades,
            winning_trades: self.winning_trades,
            win_rate: win_rate * 100.0,
            total_pnl: self.total_pnl,
            max_drawdown: self.max_drawdown,
            pnl_over_dd,
            ratio_wl,
            profit_factor,
            sharpe,
            sortino,
            avg_trade,
            avg_win,
            avg_loss,
            std_win,
            std_loss,
            largest_win: self.largest_win,
            largest_loss: self.largest_loss,
            max_consecutive_losses: self.max_consecutive_losses,
            exposure_pct,
            cpc_index,
            corr_coef,
        }
    }
}
