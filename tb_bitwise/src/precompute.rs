use dashmap::DashMap;
use std::sync::Arc;
use tb_core::ast::Expr;
use tb_math::primitives::*;
use tracing::info;
use rayon::prelude::*;

pub struct EngineCache {
    pub series: DashMap<Expr, Arc<Vec<f64>>>,
    pub bool_series: DashMap<Expr, Arc<Vec<u64>>>,
    pub data_len: usize,
    pub close: Arc<Vec<f64>>,
    pub open: Arc<Vec<f64>>,
    pub high: Arc<Vec<f64>>,
    pub low: Arc<Vec<f64>>,
    pub volume: Arc<Vec<f64>>,
    pub true_range: Arc<Vec<f64>>,
}

impl EngineCache {
    pub fn new(raw_data: &crate::data::RawData, periods: &[u32]) -> Self {
        info!("Initializing Pre-computed Engine Cache...");
        let start_time = std::time::Instant::now();
        let cache = DashMap::new();
        let bool_cache = DashMap::new();
        let data_len = raw_data.close.len();

        let close = Arc::new(raw_data.close.clone());
        let open = Arc::new(raw_data.open.clone());
        let high = Arc::new(raw_data.high.clone());
        let low = Arc::new(raw_data.low.clone());
        let volume = Arc::new(raw_data.volume.clone());
        
        let tr = Arc::new(true_range(&high, &low, &close));
        
        // Also cache them for arbitrary lookups
        cache.insert(Expr::Close, close.clone());
        cache.insert(Expr::Open, open.clone());
        cache.insert(Expr::High, high.clone());
        cache.insert(Expr::Low, low.clone());
        cache.insert(Expr::Volume, volume.clone());
        cache.insert(Expr::TrueRange, tr.clone());

        let base_series = vec![
            (Expr::Close, close.clone()),
            (Expr::High, high.clone()),
            (Expr::Low, low.clone()),
            (Expr::Volume, volume.clone()),
        ];

        // Precompute Depth-1 derived arrays for configured periods
        let depth1_series: Vec<_> = periods.par_iter().flat_map(|&p| {
            let p_usize = p as usize;
            let mut local_series = Vec::new();
            for (expr, data) in &base_series {
                // SMA
                let sma_expr = Expr::Sma { source: Box::new(expr.clone()), period: p };
                let sma_data = Arc::new(sma(data, p_usize));
                local_series.push((sma_expr, sma_data));
                
                // EMA
                let ema_expr = Expr::Ema { source: Box::new(expr.clone()), period: p };
                let ema_data = Arc::new(ema(data, p_usize));
                local_series.push((ema_expr, ema_data));
                
                // WMA
                let wma_expr = Expr::Wma { source: Box::new(expr.clone()), period: p };
                let wma_data = Arc::new(wma(data, p_usize));
                local_series.push((wma_expr, wma_data));
                
                // RMA
                let rma_expr = Expr::Rma { source: Box::new(expr.clone()), period: p };
                let rma_data = Arc::new(rma(data, p_usize));
                local_series.push((rma_expr, rma_data));
                
                // TSMAX
                let tsmax_expr = Expr::TsMax { source: Box::new(expr.clone()), period: p };
                let tsmax_data = Arc::new(ts_max(data, p_usize));
                local_series.push((tsmax_expr, tsmax_data));
                
                // TSMIN
                let tsmin_expr = Expr::TsMin { source: Box::new(expr.clone()), period: p };
                let tsmin_data = Arc::new(ts_min(data, p_usize));
                local_series.push((tsmin_expr, tsmin_data));

                // STDDEV (Only on close for base)
                if *expr == Expr::Close {
                    local_series.push((
                        Expr::StdDev { source: Box::new(expr.clone()), period: p },
                        Arc::new(std_dev(data, p_usize))
                    ));
                    local_series.push((
                        Expr::LinRegSlope { source: Box::new(expr.clone()), period: p },
                        Arc::new(lin_reg_slope(data, p_usize))
                    ));
                }
            }
            local_series
        }).collect();

        for (expr, data) in &depth1_series {
            cache.insert(expr.clone(), data.clone());
        }

        // Precompute Depth-2 derived arrays
        let mut combinations = Vec::new();
        for &p in periods {
            for (expr, data) in &depth1_series {
                combinations.push((p, expr.clone(), data.clone()));
            }
        }

        let depth2_series: Vec<_> = combinations.into_par_iter().flat_map(|(p, expr, data)| {
            let p_usize = p as usize;
            vec![
                (Expr::Sma { source: Box::new(expr.clone()), period: p }, Arc::new(sma(&data, p_usize))),
                (Expr::Ema { source: Box::new(expr.clone()), period: p }, Arc::new(ema(&data, p_usize))),
                (Expr::Wma { source: Box::new(expr.clone()), period: p }, Arc::new(wma(&data, p_usize))),
                (Expr::Rma { source: Box::new(expr.clone()), period: p }, Arc::new(rma(&data, p_usize))),
                (Expr::TsMax { source: Box::new(expr.clone()), period: p }, Arc::new(ts_max(&data, p_usize))),
                (Expr::TsMin { source: Box::new(expr.clone()), period: p }, Arc::new(ts_min(&data, p_usize))),
            ]
        }).collect();

        for (expr, data) in depth2_series {
            cache.insert(expr, data);
        }
        
        info!("Engine Cache built in {:.2?} ({} float series, {} bool series)", start_time.elapsed(), cache.len(), bool_cache.len());

        Self {
            series: cache,
            bool_series: bool_cache,
            data_len,
            close,
            open,
            high,
            low,
            volume,
            true_range: tr,
        }
    }
}
