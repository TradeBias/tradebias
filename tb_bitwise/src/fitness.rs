use crate::ga::Genome;

pub fn is_better(challenger: &Genome, king: &Genome, metric: &tb_core::fitness::FitnessFunction) -> bool {
    let challenger_score = get_fitness_score(challenger, metric);
    let king_score = get_fitness_score(king, metric);
    
    challenger_score > king_score
}

pub fn get_fitness_score(genome: &Genome, metric: &tb_core::fitness::FitnessFunction) -> f64 {
    match metric {
        tb_core::fitness::FitnessFunction::WinPercentage => genome.metrics.win_rate,
        tb_core::fitness::FitnessFunction::Pnl => genome.metrics.total_pnl,
        tb_core::fitness::FitnessFunction::AvgTrade => genome.metrics.avg_trade,
        tb_core::fitness::FitnessFunction::Drawdown => genome.metrics.max_drawdown,
        tb_core::fitness::FitnessFunction::PnlOverDd => genome.metrics.pnl_over_dd,
        tb_core::fitness::FitnessFunction::Sharpe => genome.metrics.sharpe,
        tb_core::fitness::FitnessFunction::Sortino => genome.metrics.sortino,
        tb_core::fitness::FitnessFunction::ProfitFactor => genome.metrics.profit_factor,
        tb_core::fitness::FitnessFunction::CpcIndex => genome.metrics.cpc_index,
        tb_core::fitness::FitnessFunction::CorrCoef => genome.metrics.corr_coef,
        tb_core::fitness::FitnessFunction::Cagr => genome.metrics.cagr,
        tb_core::fitness::FitnessFunction::TTest => genome.metrics.t_test,
        _ => genome.metrics.total_pnl, // Fallback
    }
}
