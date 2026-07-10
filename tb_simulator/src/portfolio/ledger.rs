use super::position::{Position, PositionSide};

#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub side: PositionSide,
    pub entry_price: f64,
    pub exit_price: f64,
    pub entry_index: usize,
    pub exit_index: usize,
    pub pnl: f64,
}

#[derive(Debug, Clone)]
pub struct Ledger {
    pub initial_balance: f64,
    pub current_balance: f64,
    pub trades: Vec<TradeRecord>,
}

impl Ledger {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            initial_balance,
            current_balance: initial_balance,
            trades: Vec::new(),
        }
    }

    /// Close an active position and record the trade
    pub fn record_trade(&mut self, position: &Position, exit_price: f64, exit_index: usize) {
        let pnl = position.unrealized_pnl(exit_price);
        self.current_balance += pnl;

        self.trades.push(TradeRecord {
            side: position.side.clone(),
            entry_price: position.entry_price,
            exit_price,
            entry_index: position.entry_index,
            exit_index,
            pnl,
        });
    }
    
    pub fn total_trades(&self) -> usize {
        self.trades.len()
    }
}
