#[derive(Debug, Clone, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub side: PositionSide,
    pub entry_price: f64,
    pub size: f64,
    pub entry_index: usize,
}

impl Position {
    pub fn new(side: PositionSide, entry_price: f64, size: f64, entry_index: usize) -> Self {
        Self {
            side,
            entry_price,
            size,
            entry_index,
        }
    }

    /// Calculate unrealized PnL at the given current price
    pub fn unrealized_pnl(&self, current_price: f64) -> f64 {
        match self.side {
            PositionSide::Long => (current_price - self.entry_price) * self.size,
            PositionSide::Short => (self.entry_price - current_price) * self.size,
        }
    }
}
