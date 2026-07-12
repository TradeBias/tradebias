use tb_core::config::Phase1Config;
use crate::state::GenerationMetrics;

pub fn export_leaderboard_to_csv(
    path: &std::path::Path,
    config: &Phase1Config,
    metrics: &GenerationMetrics,
) -> Result<(), std::io::Error> {
    let mut csv_content = String::from("AST Structure,Fitness Score,Total PnL,Expectancy,Exposure (%),Complexity,Direction,Stop Type,Take Profit,MAP-X,MAP-Y,Fitness Function,In-Sample Pct\n");
    
    // Format config options once
    let direction_str = format!("{:?}", config.trade_direction);
    let stop_str = format!("{:?}", config.stop_type);
    let tp_str = format!("{:?}", config.take_profit);
    let map_x_str = format!("{:?}", config.map_x);
    let map_y_str = format!("{:?}", config.map_y);
    let fitness_str = format!("{:?}", config.fitness);
    let is_pct_str = format!("{:.2}", config.in_sample_pct);

    for strategy in &metrics.strategies {
        let ast = strategy.sketch.to_string().replace("\"", "\"\"");
        csv_content.push_str(&format!(
            "\"{}\",{:.4},{:.2},{:.4},{:.1},{},{},{},{},{},{},{},{}\n",
            ast,
            strategy.fitness,
            strategy.pnl,
            strategy.expectancy,
            strategy.trade_frequency,
            strategy.indicator_count,
            direction_str,
            stop_str,
            tp_str,
            map_x_str,
            map_y_str,
            fitness_str,
            is_pct_str
        ));
    }
    
    std::fs::write(path, csv_content)
}
