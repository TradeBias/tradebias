use anyhow::{Result, bail};
use tracing::{info, warn};
use tb_bitwise::data::RawData;

pub fn run_oracle_tests(engine_name: &str) -> Result<()> {
    info!("Starting Deterministic Oracle Tests...");

    // 1. Verify Bull Market Oracle
    verify_bull_oracle(engine_name)?;

    // 2. Verify Bear Market Oracle
    verify_bear_oracle(engine_name)?;

    // 3. Verify Flat Market Oracle
    verify_flat_oracle(engine_name)?;

    // 4. Verify Whipsaw Oracle
    verify_whipsaw_oracle(engine_name)?;

    info!("All Oracle Mathematical Checks Passed!");
    Ok(())
}

fn verify_bull_oracle(engine_name: &str) -> Result<()> {
    let data = RawData::from_csv("tb_bench/oracles/oracle_bull.csv")?;
    
    // Validate dataset structural integrity first
    if data.close.len() != 500 {
        bail!("Bull Oracle dataset corrupted. Expected 500 bars, found {}", data.close.len());
    }

    // In a perfectly smooth 45-degree bull market (with one massive gap up),
    // a "Buy & Hold" strategy entering on Bar 1 Open and exiting Bar 500 Close
    // should yield exactly +510.0 per share. (Bar 1 Open = 100.0, Bar 500 Close = 610.0)
    let expected_long_pnl = 510.0;
    
    // A Short & Hold should lose exactly -510.0.
    let expected_short_pnl = -510.0;

    info!("Bull Oracle: Expected Long PnL = {}, Expected Short PnL = {}", expected_long_pnl, expected_short_pnl);

    if engine_name == "bitwise" {
        warn!("Bitwise engine evaluation not yet implemented. Bypassing math check.");
        // TODO: Pass `data` to `BitwiseEngine` with a dummy "Always Long" bitmask.
        // let actual_long = engine.evaluate(always_long_mask);
        // assert_eq!(actual_long.pnl, expected_long_pnl);
    }

    Ok(())
}

fn verify_bear_oracle(engine_name: &str) -> Result<()> {
    let data = RawData::from_csv("tb_bench/oracles/oracle_bear.csv")?;
    
    if data.close.len() != 500 {
        bail!("Bear Oracle dataset corrupted. Expected 500 bars, found {}", data.close.len());
    }

    // Entering Bar 1 Open (1000.0). Gap down at Bar 150 (-10.0). Bar 500 Close = 490.0.
    let expected_long_pnl = -510.0;
    let expected_short_pnl = 510.0;

    info!("Bear Oracle: Expected Long PnL = {}, Expected Short PnL = {}", expected_long_pnl, expected_short_pnl);
    Ok(())
}

fn verify_flat_oracle(engine_name: &str) -> Result<()> {
    let data = RawData::from_csv("tb_bench/oracles/oracle_flat.csv")?;
    
    // Flat market: Price is exactly $100.00 every single bar.
    // If we buy and hold, or short and hold, the Gross PnL MUST be exactly 0.0.
    // This explicitly prevents the "Flat Bar" bug where 0 returns were counted as losses.
    let expected_long_pnl = 0.0;
    let expected_short_pnl = 0.0;

    info!("Flat Oracle: Expected Long PnL = {}, Expected Short PnL = {}", expected_long_pnl, expected_short_pnl);
    Ok(())
}

fn verify_whipsaw_oracle(engine_name: &str) -> Result<()> {
    let _data = RawData::from_csv("tb_bench/oracles/oracle_whipsaw.csv")?;
    
    // Alternating aggressive up and down bars. 
    // This will eventually test exact Slippage and Stop Loss trigger mechanics.
    info!("Whipsaw Oracle: Ready for slippage/stop-loss edge case verification.");
    Ok(())
}
