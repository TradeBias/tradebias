use crate::ga::Genome;

pub struct MapArchive {
    pub grid: Vec<Vec<Option<Genome>>>,
    pub grid_size: usize,
    pub x_trait: tb_core::archive::ArchiveTrait,
    pub y_trait: tb_core::archive::ArchiveTrait,
    pub fitness_metric: tb_core::fitness::FitnessFunction,
    pub max_candles: u32,
    pub max_complexity: usize,
}

impl MapArchive {
    pub fn new(x_trait: tb_core::archive::ArchiveTrait, y_trait: tb_core::archive::ArchiveTrait, fitness_metric: tb_core::fitness::FitnessFunction, grid_size: usize, max_candles: u32, max_complexity: usize) -> Self {
        Self {
            grid: vec![vec![None; grid_size]; grid_size],
            grid_size,
            x_trait,
            y_trait,
            fitness_metric,
            max_candles,
            max_complexity,
        }
    }

    fn get_bin(&self, t: &tb_core::archive::ArchiveTrait, genome: &Genome) -> usize {
        let pct = match t {
            tb_core::archive::ArchiveTrait::WinRate => genome.metrics.win_rate,
            tb_core::archive::ArchiveTrait::MarketExposure => {
                if self.max_candles == 0 { 0.0 } else { 
                    (genome.metrics.total_trades as f64 / self.max_candles as f64) * 100.0
                }
            },
            tb_core::archive::ArchiveTrait::Complexity => {
                let comp = genome.conditions.len();
                if self.max_complexity <= 1 { 0.0 } else {
                    ((comp.saturating_sub(1)) as f64 / (self.max_complexity - 1) as f64) * 100.0
                }
            },
            tb_core::archive::ArchiveTrait::MaxDrawdown => 0.0, // Stubbed until PnL logic is merged
        };
        
        let mut bin = (pct / (100.0 / self.grid_size as f64)) as usize;
        if bin >= self.grid_size {
            bin = self.grid_size - 1;
        }
        bin
    }

    pub fn submit(&mut self, genome: Genome) -> bool {
        if genome.metrics.total_trades == 0 { return false; }
        
        let x = self.get_bin(&self.x_trait, &genome);
        let y = self.get_bin(&self.y_trait, &genome);

        let is_better = match &self.grid[x][y] {
            Some(king) => crate::fitness::is_better(&genome, king, &self.fitness_metric),
            None => true,
        };

        if is_better {
            self.grid[x][y] = Some(genome);
            true
        } else {
            false
        }
    }
}
