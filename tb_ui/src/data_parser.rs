use polars::prelude::*;

pub fn parse_dataframe_to_bars(df: &DataFrame) -> Result<egui_charts::model::BarData, String> {
    let open_s = df.column("open").map_err(|_| "Missing open")?.cast(&DataType::Float64).map_err(|e| e.to_string())?;
    let high_s = df.column("high").map_err(|_| "Missing high")?.cast(&DataType::Float64).map_err(|e| e.to_string())?;
    let low_s = df.column("low").map_err(|_| "Missing low")?.cast(&DataType::Float64).map_err(|e| e.to_string())?;
    let close_s = df.column("close").map_err(|_| "Missing close")?.cast(&DataType::Float64).map_err(|e| e.to_string())?;
    let vol_s = df.column("volume").map_err(|_| "Missing volume")?.cast(&DataType::Float64).map_err(|e| e.to_string())?;

    let open = open_s.f64().unwrap();
    let high = high_s.f64().unwrap();
    let low = low_s.f64().unwrap();
    let close = close_s.f64().unwrap();
    let volume = vol_s.f64().unwrap();

    let time_col = df.column("timestamp").map_err(|_| "Missing timestamp")?;
    
    let time = if time_col.dtype() == &DataType::String {
        let mut vec = Vec::with_capacity(time_col.len());
        for opt_s in time_col.str().map_err(|e| e.to_string())?.into_iter() {
            if let Some(s) = opt_s {
                let s = s.replace(".", "-");
                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                    vec.push(Some(dt.and_utc().timestamp_millis()));
                } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M") {
                    vec.push(Some(dt.and_utc().timestamp_millis()));
                } else {
                    vec.push(None);
                }
            } else {
                vec.push(None);
            }
        }
        let chunked = Int64Chunked::new("timestamp".into(), &vec);
        chunked.into_series().cast(&DataType::Datetime(TimeUnit::Milliseconds, None)).unwrap().datetime().unwrap().clone()
    } else if matches!(time_col.dtype(), DataType::Int32 | DataType::Int64) {
        time_col.cast(&DataType::Datetime(TimeUnit::Milliseconds, None)).map_err(|e| e.to_string())?.datetime().unwrap().clone()
    } else {
        time_col.cast(&DataType::Datetime(TimeUnit::Microseconds, None)).map_err(|e| e.to_string())?.datetime().unwrap().clone()
    };

    let mut bars = Vec::with_capacity(df.height());
    for i in 0..df.height() {
        let t_val = time.get(i).ok_or("Null timestamp")?;
        
        bars.push(egui_charts::model::Bar {
            time: chrono::DateTime::from_timestamp_millis(t_val).unwrap_or_default(),
            open: open.get(i).unwrap_or(0.0),
            high: high.get(i).unwrap_or(0.0),
            low: low.get(i).unwrap_or(0.0),
            close: close.get(i).unwrap_or(0.0),
            volume: volume.get(i).unwrap_or(0.0),
        });
    }
    
    Ok(egui_charts::model::BarData { bars })
}
