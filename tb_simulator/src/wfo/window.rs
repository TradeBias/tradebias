#[derive(Debug, Clone)]
pub struct WfoConfig {
    pub is_window_size: usize,  // In-sample data length (number of rows)
    pub oos_window_size: usize, // Out-of-sample data length (number of rows)
    pub anchored: bool,         // True if IS start is anchored to index 0
}

impl WfoConfig {
    pub fn new(is_window_size: usize, oos_window_size: usize, anchored: bool) -> Self {
        Self {
            is_window_size,
            oos_window_size,
            anchored,
        }
    }
}

pub struct WfoSlicer {
    config: WfoConfig,
    total_rows: usize,
    current_step: usize,
}

impl WfoSlicer {
    pub fn new(config: WfoConfig, total_rows: usize) -> Self {
        Self {
            config,
            total_rows,
            current_step: 0,
        }
    }
}

/// Represents a tuple of (In-Sample Start, IS Length, Out-of-Sample Start, OOS Length)
pub type WfoSlice = (usize, usize, usize, usize);

impl Iterator for WfoSlicer {
    type Item = WfoSlice;

    fn next(&mut self) -> Option<Self::Item> {
        let is_start = if self.config.anchored {
            0
        } else {
            self.current_step * self.config.oos_window_size
        };

        // If anchored, the IS window grows. If rolling, it stays fixed.
        let is_length = if self.config.anchored {
            self.config.is_window_size + (self.current_step * self.config.oos_window_size)
        } else {
            self.config.is_window_size
        };

        let oos_start = is_start + is_length;
        let oos_length = self.config.oos_window_size;

        // Ensure we have enough data for the OOS slice
        if oos_start + oos_length > self.total_rows {
            return None; // Reached the end of the dataset
        }

        self.current_step += 1;
        Some((is_start, is_length, oos_start, oos_length))
    }
}
