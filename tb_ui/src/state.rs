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
    pub right_panel_open: bool,
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
    pub raw_df_cache: Option<(DataFrame, PathBuf)>,
    pub available_columns: Vec<String>,
    pub column_mapping: ColumnMapping,
    
    // Regime Highlighting State
    pub is_dragging_regime: bool,
    pub current_regime_start: Option<usize>,
    pub selected_regimes: Vec<(usize, usize)>,
    
    // Channels
    pub elite_tx: Option<Sender<EliteStrategy>>,
    pub elite_rx: Option<Receiver<EliteStrategy>>,
    pub foundry_rx: Option<Receiver<GenerationMetrics>>,
    pub latest_metrics: Option<GenerationMetrics>,
    
    // WFO State (Live Feed from Phase 1)
    pub wfo_rx: Option<Receiver<Result<(EliteStrategy, tb_simulator::metrics::TearSheet), String>>>,
    pub wfo_results: Vec<(EliteStrategy, tb_simulator::metrics::TearSheet)>,
    pub wfo_running: bool,
    
    // WFO State (Dedicated Simulator Tab)
    pub simulator_wfo_rx: Option<Receiver<Result<tb_simulator::metrics::TearSheet, String>>>,
    pub latest_simulator_tearsheet: Option<tb_simulator::metrics::TearSheet>,
    
    // Persisted Engine for Robustness Testing
    pub bitwise_engine: Option<std::sync::Arc<tb_bitwise::engine::BitwiseEngine>>,
    pub engine_tx: Option<Sender<std::sync::Arc<tb_bitwise::engine::BitwiseEngine>>>,
    pub engine_rx: Option<Receiver<std::sync::Arc<tb_bitwise::engine::BitwiseEngine>>>,
}

impl Default for TradingApp {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            right_panel_open: true,
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
            raw_df_cache: None,
            available_columns: vec![],
            column_mapping: ColumnMapping::default(),
            is_dragging_regime: false,
            current_regime_start: None,
            selected_regimes: vec![],
            elite_tx: None,
            elite_rx: None,
            foundry_rx: None,
            latest_metrics: None,
            wfo_rx: None,
            wfo_results: vec![],
            wfo_running: false,
            simulator_wfo_rx: None,
            latest_simulator_tearsheet: None,
            bitwise_engine: None,
            engine_tx: None,
            engine_rx: None,
        }
    }
}

impl TradingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
