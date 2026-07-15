use eframe::egui;
use egui_charts::{ChartBuilder, TradingChart, model::Timeframe, theme::Theme};
use polars::prelude::*;
use std::path::PathBuf;
use crossbeam_channel::{Sender, Receiver};
use tb_core::ast::EliteStrategy;

#[derive(Clone, Debug)]
pub struct GenerationMetrics {
    pub generation: usize,
    pub elapsed_seconds: f64,
    pub total_generated: usize,
    pub total_discarded: usize,
    pub strategies: Vec<EliteStrategy>,
    pub status_msg: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CustomVariable {
    pub name: String,
    pub min: u32,
    pub max: u32,
    pub step: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pill {
    Source(String),            // e.g. "Close", "Open"
    Indicator(String, String), // e.g. "SMA", "Var_1" (Points to the CustomVariable name for period)
    Operator(String),          // e.g. "+", "-", "*", "/"
    Constant(f64),             // e.g. 2.0
    OpenBracket,               // "("
    CloseBracket,              // ")"
}

pub struct ColumnMapping {
    pub time: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

impl Default for ColumnMapping {
    fn default() -> Self {
        Self {
            time: "timestamp".into(),
            open: "open".into(),
            high: "high".into(),
            low: "low".into(),
            close: "close".into(),
            volume: "volume".into(),
        }
    }
}

pub struct TradingApp {
    pub selected_tab: usize,
    pub _right_panel_open: bool,
    pub config: tb_core::SessionConfig,
    pub main_chart: TradingChart,
    pub loaded_data: Option<LazyFrame>,
    
    // Column Mapping State
    pub show_mapping_modal: bool,
    pub show_indicator_modal: bool,
    pub show_metric_filters_modal: bool,
    
    // Robustness State
    pub show_robustness_modal: bool,
    pub selected_strategy_idx: Option<usize>,
    pub robustness_report: Option<tb_bitwise::robustness::RobustnessReport>,
    pub robustness_noise_pct: f64,
    pub robustness_top_n_drop: usize,
    pub robustness_show_mc: bool,
    pub robustness_show_noise: bool,
    pub robustness_show_deletion: bool,
    pub robustness_disabled_conditions: std::collections::HashSet<usize>,
    
    // Metric Filters
    pub min_pnl_filter: f64,
    pub max_pnl_filter: f64,
    pub min_win_rate_filter: f64,
    pub max_win_rate_filter: f64,
    pub min_corr_coef_filter: f64,
    pub max_cons_loss_filter: usize,
    pub raw_df_cache: Option<(DataFrame, PathBuf)>,
    pub available_columns: Vec<String>,
    pub column_mapping: ColumnMapping,
    
    // Regime Highlighting    // CPCV State (Now Unused)
    pub _is_dragging_regime: bool,
    pub _current_regime_start: Option<usize>,
    pub _selected_regimes: Vec<(usize, usize)>,
    
    // Channels
    pub elite_tx: Option<Sender<EliteStrategy>>,
    pub elite_rx: Option<Receiver<EliteStrategy>>,
    pub foundry_rx: Option<Receiver<GenerationMetrics>>,
    pub latest_metrics: Option<GenerationMetrics>,
    
    // Phase 2 State
    pub _wfo_rx: Option<Receiver<Result<(EliteStrategy, tb_simulator::metrics::TearSheet), String>>>,
    pub _wfo_results: Vec<(EliteStrategy, tb_simulator::metrics::TearSheet)>,
    pub wfo_running: bool,
    
    // WFO State (Dedicated Simulator Tab)
    pub simulator_wfo_rx: Option<Receiver<Result<tb_simulator::metrics::TearSheet, String>>>,
    pub latest_simulator_tearsheet: Option<tb_simulator::metrics::TearSheet>,
    
    pub bitwise_engine: Option<std::sync::Arc<tb_bitwise::engine::BitwiseEngine>>,
    pub engine_tx: Option<Sender<std::sync::Arc<tb_bitwise::engine::BitwiseEngine>>>,
    pub engine_rx: Option<Receiver<std::sync::Arc<tb_bitwise::engine::BitwiseEngine>>>,
    
    // Indicator Forge State
    pub forge_blueprints: Vec<tb_core::ast::IndicatorBlueprint>,
    pub forge_active_blueprint_name: String,
    pub forge_active_type: tb_core::ast::SemanticType,
    pub forge_node_graph: crate::node_graph::NodeGraphState,
    pub forge_graph_history: Vec<(String, crate::node_graph::NodeGraphState)>,
    pub forge_show_chart: bool,
    pub forge_last_ast_str: String,
}

impl TradingApp {
    pub fn rebuild_engine_if_data_loaded(&mut self) {
        if let Some(lf) = &self.loaded_data {
            if let Ok(df) = lf.clone().collect() {
                use polars::prelude::*;
                let opens: Vec<f64> = df.column("open").unwrap().cast(&DataType::Float64).unwrap().f64().unwrap().into_no_null_iter().collect();
                let highs: Vec<f64> = df.column("high").unwrap().cast(&DataType::Float64).unwrap().f64().unwrap().into_no_null_iter().collect();
                let lows: Vec<f64> = df.column("low").unwrap().cast(&DataType::Float64).unwrap().f64().unwrap().into_no_null_iter().collect();
                let closes: Vec<f64> = df.column("close").unwrap().cast(&DataType::Float64).unwrap().f64().unwrap().into_no_null_iter().collect();
                let volumes: Vec<f64> = if let Ok(vol) = df.column("volume").or_else(|_| df.column("Volume")) {
                    vol.cast(&DataType::Float64).unwrap().f64().unwrap().into_no_null_iter().collect()
                } else {
                    vec![0.0; closes.len()]
                };

                let raw_data = tb_bitwise::data::RawData::from_arrays(opens.clone(), highs.clone(), lows.clone(), closes.clone(), volumes.clone()).unwrap();
                let cache = tb_bitwise::precompute::EngineCache::new(&raw_data, &[7, 14, 21, 50, 100, 200]);
                let arc_cache = std::sync::Arc::new(cache);

                let atr_data = match (&self.config.phase1.stop_type, &self.config.phase1.take_profit) {
                    (tb_core::stops::StopType::StandardStop { calc: tb_core::stops::StopCalculation::Atr { .. } }, _) |
                    (tb_core::stops::StopType::TrailingStop { calc: tb_core::stops::StopCalculation::Atr { .. } }, _) |
                    (_, tb_core::stops::TakeProfit::Atr { .. }) => {
                        let tr = tb_math::primitives::true_range(&highs, &lows, &closes);
                        Some(tb_math::primitives::rma(&tr, 14))
                    }
                    _ => None,
                };
                
                let targets = tb_bitwise::targets::TargetOutcomes::generate_advanced_stop(
                    &raw_data, 
                    &self.config.phase1.stop_type, 
                    &self.config.phase1.take_profit,
                    atr_data.as_deref(),
                    self.config.phase1.slippage_penalty
                );
                let arc_targets = std::sync::Arc::new(targets);

                self.bitwise_engine = Some(std::sync::Arc::new(tb_bitwise::engine::BitwiseEngine::new(arc_cache, arc_targets)));
                println!("Global BitwiseEngine rebuilt successfully.");
            }
        }
    }
}

impl Default for TradingApp {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            _right_panel_open: true,
            config: tb_core::SessionConfig::default(),
            main_chart: ChartBuilder::new()
                .with_symbol("BTC-USD")
                .with_timeframe(Timeframe::Hour1)
                .with_theme(Theme::dark())
                .build(),
            loaded_data: None,
            show_mapping_modal: false,
            show_indicator_modal: false,
            show_metric_filters_modal: false,
            show_robustness_modal: false,
            selected_strategy_idx: None,
            robustness_report: None,
            robustness_noise_pct: 0.5,
            robustness_top_n_drop: 3,
            robustness_show_mc: true,
            robustness_show_noise: true,
            robustness_show_deletion: true,
            robustness_disabled_conditions: std::collections::HashSet::new(),
            min_pnl_filter: 0.0,
            max_pnl_filter: 100_000_000.0,
            min_win_rate_filter: 0.0,
            max_win_rate_filter: 100.0,
            min_corr_coef_filter: -1.0,
            max_cons_loss_filter: 1000,
            raw_df_cache: None,
            available_columns: vec![],
            column_mapping: ColumnMapping::default(),
            _is_dragging_regime: false,
            _current_regime_start: None,
            _selected_regimes: Vec::new(),
            elite_tx: None,
            elite_rx: None,
            foundry_rx: None,
            latest_metrics: None,
            // Simulator (Phase 2)
            _wfo_rx: None,
            _wfo_results: Vec::new(),
            wfo_running: false,
            simulator_wfo_rx: None,
            latest_simulator_tearsheet: None,
            
            bitwise_engine: None,
            engine_tx: None,
            engine_rx: None,
            
            forge_blueprints: tb_indicators::templates::default_blueprints(),
            forge_active_blueprint_name: "New_Indicator".to_string(),
            forge_active_type: tb_core::ast::SemanticType::Price,
            forge_node_graph: crate::node_graph::NodeGraphState::default(),
            forge_graph_history: Vec::new(),
            forge_show_chart: false,
            forge_last_ast_str: String::new(),
        }
    }
}

impl TradingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
