use tb_core::config::Phase1Config;
use crate::state::GenerationMetrics;

pub fn export_leaderboard_to_csv(
    path: &std::path::Path,
    config: &Phase1Config,
    metrics: &GenerationMetrics,
) -> Result<(), std::io::Error> {
    let mut csv_content = String::from("AST Structure,Fitness Score,Total PnL,Avg Trade,Avg Win,Avg Loss,Largest Win,Largest Loss,Std Win,Std Loss,Max DD,PnL/DD,Max Cons Loss,Sharpe,Sortino,Profit Factor,CPC Index,Corr Coef,Exposure (%),Complexity,Direction,Stop Type,Take Profit,MAP-X,MAP-Y,Fitness Function,In-Sample Pct\n");
    
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
            "\"{}\",{:.4},{:.2},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.1},{},{},{},{},{},{},{},{}\n",
            ast,
            strategy.fitness,
            strategy.pnl,
            strategy.avg_trade,
            strategy.avg_win,
            strategy.avg_loss,
            strategy.largest_win,
            strategy.largest_loss,
            strategy.std_win,
            strategy.std_loss,
            strategy.max_drawdown,
            strategy.pnl_over_dd,
            strategy.max_consecutive_losses,
            strategy.sharpe,
            strategy.sortino,
            strategy.profit_factor,
            strategy.cpc_index,
            strategy.corr_coef,
            strategy.exposure_pct,
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
