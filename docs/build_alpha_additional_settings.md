# Build Alpha - Additional Settings Documentation

This document explains the various configuration options available in the Build Alpha Strategy Builder, specifically focusing on the Additional Settings panel and related configuration sections.

## Additional Settings Panel

### Position and Costs
* **Position Sizing Mode**: Determines how trade sizes are calculated (e.g., Default, Fixed Fractional, Volatility Adjusted).
* **Use Position Sizing Scaler**: Allows scaling the position size dynamically based on a custom multiplier or condition.
* **Slippage Mode**: Defines how slippage is applied to trades (e.g., Per Trade, Per Share/Contract).
* **Commission Mode**: Defines how trading commissions are calculated (e.g., Per Trade, Per Share/Contract).
* **Slippage (market currency)**: The estimated amount of slippage incurred per trade, denominated in the market currency.
* **Commission (market currency)**: The estimated commission cost per trade, denominated in the market currency.
* **Account Value**: The starting capital for the backtest or simulation.

### Entry and Exit Execution
* **Entry On**: Specifies when a trade should be entered after a signal is generated (e.g., Next Open).
* **Exit On**: Specifies when a trade should be exited (e.g., Close).
* **Out of Sample**: Allocates data for out-of-sample testing to prevent overfitting. You can designate a portion of data at the Beginning or End of the historical dataset.
* **Delayed Entry**: Delays the entry by a specified number of bars/periods after the signal.
* **ATR Exit Length**: The lookback period used if an Average True Range (ATR) based trailing stop or exit is applied.

### Machine Generated Rules
* **Use Machine Generated Rules**: Toggles whether the engine should automatically generate and inject machine-learned rules into the strategies.
* **Lookback for Machine Generated Rules**: The historical window length the machine learning algorithm uses to discover patterns.
* **Number of Machine Generated Rules**: The maximum number of auto-generated rules the system will attempt to create.

### Compound Account Settings
* **Use compound account**: Toggles compounding of returns. When enabled, profits are reinvested into subsequent trades.
* **Account sharing**: Determines how capital is shared among multiple strategies or symbols (e.g., Full).
* **In-trade position adjustment**: Allows adjusting the size of an open position while the trade is active.
* **In-trade adjustment threshold**: The percentage threshold that triggers an in-trade position adjustment.
* **Position sizing**: The method for calculating position sizes when compounding (e.g., Percentage of equity).
* **Concurrent sizing**: Determines how capital is allocated when multiple entry signals occur simultaneously (e.g., Queue Based).
* **Max exposure**: The maximum percentage of total account equity that can be at risk at any given time.
* **Max positions**: The maximum number of open positions allowed concurrently.
* **Min position size**: The minimum acceptable size for a trade.
* **Position exit mode**: Defines how positions are scaled out or closed (e.g., Standard).

---

## Main Panel Settings (Reference)

### General & Data
* **Target currency**: The base currency for the simulation and results (e.g., USD).
* **Invest Cash Symbol**: Symbol used for holding cash (e.g., a money market fund).
* **Vs. Other Symbol 1/2/3**: Allows comparing or correlating the primary strategy against up to three secondary symbols.
* **Calculate Vs. Random**: Compares strategy performance against randomly generated entries/exits to test statistical significance.
* **Use custom strategies**: Enables the use of user-defined custom strategy logic.
* **Force end of day exit**: Closes all open positions at the end of the trading session.
* **Session end time**: Defines the time for end-of-day exits (e.g., 5:00 PM).
* **Max trades per day**: Limits the number of round-trip trades allowed within a single day.
* **Max Rules per strategy**: Limits the complexity of generated strategies by capping the number of conditions.
* **Exit signals Mode**: Defines how multiple exit signals are handled (e.g., Single).
* **LISS mode**: Look-Inside-Bar Simulator mode for finer intra-bar precision.

### Rebalance Options
* **Symbols to trade**: Number of symbols to actively trade.
* **Rebalance Frequency**: How often the portfolio weights are adjusted (e.g., Never).
* **Rebalance Method**: The mathematical method for rebalancing (e.g., Profit Factor).

### Continuous Simulation
* **Entry rules Max count**: Maximum number of entry rules allowed in continuous simulation mode.
* **Exit rules Max count**: Maximum number of exit rules allowed in continuous simulation mode.

### Shift Settings
* **Shift Mode**: Specifies the timeframe shifting mode (e.g., Day).
