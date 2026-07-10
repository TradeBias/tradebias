# Alpha Target Engineering: Phase 1 Fitness

> **Context:** In a two-phase backtest pipeline where Phase 1 uses matrix multiplication to evaluate millions of strategies, how do we construct the target vector? The goal is to optimize for **Risk-Adjusted Forward Returns** while actively penalizing **Intra-trade Drawdown (MAE)**, without requiring the GA to mutate complex exit parameters.

Because Phase 1 uses a `Signal Matrix * Target Vector` operation, all of the complexity must be baked into how we construct the `Target Vector` *before* the evolution loop begins.

Here is a brainstorm of the best approaches for constructing that target, ranked by how well they balance Alpha discovery with MAE penalization.

---

## 1. The "Sharpe Vector" (Volatility-Normalized Returns)
**Focus:** Pure Risk-Adjusted Returns

Instead of targeting raw percentage returns, you divide the forward returns by the asset's rolling volatility (ATR or Standard Deviation). 

* **The Math:** `Target[i] = Forward_Return[i..i+N] / Rolling_ATR[i]`
* **Why it fits:** It prevents the GA from finding strategies that only work during massive, chaotic market crashes. A 1% gain in a quiet market is rewarded more heavily than a 2% gain in a wildly volatile market. It natively enforces risk-adjustment at the bar level.

## 2. The Drawdown-Penalized Integral (The Ulcer Target)
**Focus:** Heavy MAE Penalization

Instead of looking at the final price at Bar `N`, you evaluate the entire journey over the `N` bars, explicitly subtracting the Maximum Adverse Excursion (the deepest drawdown from the entry price).

* **The Math:** `Target[i] = Sum(Returns[i..i+N]) - (Penalty_Weight * MAE[i..i+N])`
* **Why it fits:** This is the most direct way to penalize MAE. If a breakout strategy goes -5% before ripping +10%, the MAE penalty destroys its score. The GA will naturally gravitate towards strategies that experience zero heat (immediate profit).

## 3. The Gaussian (Bell Curve) Horizon Target
**Focus:** Ignoring Market Noise & Retests

Exponential decay punishes throwbacks (retests of a breakout). A Gaussian curve ignores the immediate noise and focuses the reward on a specific horizon.

* **The Math:** Multiply the N-bar forward returns by a Bell Curve of weights (e.g., peak weight at Bar 15, fading to 0 at Bar 1 and Bar 30).
* **Why it fits:** It allows you to target specific trading styles (Scalp vs Swing) without hardcoding a strict exit. By placing low weight on Bars 1-3, you give the trade "breathing room" to endure a minor MAE without destroying the strategy's fitness.

## 4. Multi-Horizon Composite Scoring (The Robustness Matrix)
**Focus:** Eliminating Curve-Fitting to a Specific Horizon

Instead of multiplying the signal by one target vector, you multiply it by three separate vectors (e.g., 5-bar, 15-bar, and 45-bar forward returns). 

* **The Math:** `Fitness = Harmonic_Mean(Score_5, Score_15, Score_45)`
* **Why it fits:** If a strategy is only profitable at exactly 15 bars, it is likely overfit curve-junk. A truly robust alpha signal will show predictive edge across multiple timeframes. This guarantees the elites passed to Phase 2 are undeniably robust.

## 5. The "Virtual TP/SL" Binary Target
**Focus:** Maximum Phase 2 Compatibility

Even though we aren't executing stateful stops in Phase 1, we can pre-calculate a "Virtual" Take-Profit and Stop-Loss for every single bar on the chart based on multiples of ATR.

* **The Math:** For every bar `i`, look forward. Does the price hit `+2 ATR` before it hits `-1 ATR`? If Yes, `Target[i] = 1`. If No, `Target[i] = -1`. 
* **Why it fits:** The matrix math evaluates the GA purely on its **Hit Rate**. The GA is optimizing for: *"When this signal fires, what is the probability it hits a 2R profit before a 1R loss?"* This perfectly mimics Phase 2 risk management without requiring a stateful portfolio loop.

## 6. The Sortino Matrix (Downside Deviation)
**Focus:** Asymmetric Risk (Pennies in front of a steamroller)

Sharpe ratio penalizes all volatility (both upside and downside). Sortino only penalizes downside variance.

* **The Math:** Separate the matrix evaluation into `Positive_Returns` and `Negative_Returns`. `Fitness = Mean(Positive) / RMS(Negative)`.
* **Why it fits:** In trend-following, upside volatility is exactly what you want. Sortino allows the GA to hunt for massive, explosive breakouts while aggressively killing strategies that suffer from deep, sudden downside spikes.

---

## 7. Anti-Curve-Fitting: Combinatorial Purged Cross-Validation (CPCV)
**Focus:** Mathematically proving edge robustness at 10 million evaluations per second.

Since a Genetic Algorithm is literally a machine designed to repeat backtests millions of times, we *must* front-load an anti-curve-fitting mechanism into the Phase 1 matrix math. 

### CPCV via Matrix Slicing
Instead of evaluating the matrix over the entire dataset, we pre-slice the `Target Vector` into $N$ chronological chunks (e.g., 6 chunks). 
* **The Math:** The matrix multiplication evaluates the strategy against all combinations of those chunks (e.g., train on 4, test on 2). 
* **The Speed:** This is just slicing the Polars DataFrame before the dot product. It adds almost zero overhead.
* **The Result:** Phase 1 fitness is not just "Total Return". It is the *variance of performance across the CPCV paths*. If a strategy makes 100% in path A, but -20% in path B, it is curve-fit. The GA will hunt for strategies with a flat, consistent performance profile across all out-of-sample combinations.

---

### The Ultimate Phase 1 Pipeline Blueprint

If we want to build the absolute most robust Phase 1 possible without causing constraint collapse (the "Empty Set" problem), we must combine these methods using a **Pareto Frontier (Multi-Objective)** approach. 

Here is the exact step-by-step pipeline for Phase 1:

#### Step 1: Pre-Processing (Indicator Generation)
Before the GA matrix math starts, `tb_foundry` computes all required indicators defined by the generated ASTs. It uses Polars to rapidly compute these columns and joins them to the core `MasterDataset` (which `tb_data` provides, containing raw OHLCV and ATR). This guarantees indicators are computed exactly once per generation, avoiding redundant math.

#### Step 2: The Target Matrix (Virtual TP/SL + Multi-Horizon)
We do not use raw returns. We pre-compute a **Virtual TP/SL Binary Target** across 3 different risk profiles (e.g., Tight, Medium, Wide). 
* For every bar on the chart, the target is `1` if the price hits a +2 ATR target before a -1 ATR stop, and `-1` otherwise. 
* This explicitly forces the GA to hunt for signals that have a high probability of surviving Phase 2's stateful execution.

#### Step 3: The Matrix Multiplication (CPCV Sliced)
The GA generates a population of 100,000 strategies (the Signal Matrix). We slice both the Signal Matrix and the Target Matrix into 6 chronological blocks. 
* We perform the matrix multiplication to get the strategy performance across all Combinatorial Purged Cross-Validation (CPCV) folds simultaneously.

#### Step 4: Hard Constraints & The Pareto Sort
Before the Pareto sort begins, all strategies are subjected to **Hard Constraints** (e.g., must fire > 50 trades, exposure must be < 40%). Any strategy that violates these constraints is instantly assigned a fitness of `-9999` and killed. 

For the surviving strategies, we calculate 3 distinct objectives and push them to a strict NSGA-II Pareto sort:

* **Objective 1 (Raw Edge):** Maximize the **Continuous Cumulative Return** of the Virtual TP/SL execution. (Using continuous return rather than a binary hit-rate prevents the "step-function" problem and provides a smooth mathematical gradient for the GA to climb).
* **Objective 2 (Regime Robustness):** Minimize **CPCV Variance**. (Ensures the strategy didn't just get lucky in one specific bull market. Relies on a diverse chronological dataset to guarantee safety).
* **Objective 3 (Dynamic User Goal):** This slot maps to the user's "Optimization Focus". It can be *Minimize AST Complexity*, *Minimize MAE*, or *Maximize Win/Loss Ratio*. This prevents Constraint Collapse while giving the user directional control.

#### The Output
The GA does not pass "the best" strategy to Phase 2. It passes the **Rank 0 Pareto Frontier**—a diverse set of elites that are mathematically proven to be stationary, regime-robust, and highly likely to survive ATR-based stops. Phase 2 then takes over to simulate realistic slippage and compounding!

---

## 8. Safe Configuration Abstractions (Retail UI)

To protect retail users from breaking the mathematical integrity of the De Prado anti-overfitting pipeline (e.g., causing constraint collapse by setting conflicting Pareto weights), the UI should **abstract the math away completely**. 

Users should only configure the *personality* of the strategies they want. The system translates these high-level goals into safe matrix parameter shifts under the hood:

### 1. Trading Style (The Horizon Slider)
* **User selects:** "Scalping (Fast)", "Swing Trading (Medium)", or "Position Trading (Slow)".
* **Under the hood:** This shifts the `Virtual TP/SL` risk profiles. For Scalping, the matrix evaluates for 0.5 ATR targets. For Position Trading, it evaluates for 3.0 ATR targets. The math stays perfectly robust; it just targets a different timescale.

### 2. Risk Appetite (The Pareto Weight)
* **User selects:** "Conservative (Capital Preservation)" vs "Aggressive (Max Growth)".
* **Under the hood:** Adjusts how the final "Rank 0" Pareto frontier is filtered before Phase 2. Conservative mode guarantees only strategies with the absolute lowest CPCV variance pass through, while Aggressive mode allows strategies with higher Raw Returns to pass even if they have slightly more variance across macro-regimes.

### 3. Optimization Focus (The Dynamic 3rd Pareto Slot)
To prevent "Constraint Collapse" (The Curse of Dimensionality), the GA uses a strict 3-objective Pareto sort. The first two are fixed to guarantee safety (Maximize Edge, Minimize CPCV Variance). The user controls the 3rd objective via their stated goal:
* **"Keep it Simple":** Slot 3 becomes *Minimize AST Complexity*. Forces the GA to find the most elegant, human-readable logic.
* **"Protect my Downside":** Slot 3 becomes *Minimize MAE (Maximum Adverse Excursion)*. Forces the GA to find entries that immediately go into profit without suffering heat.
* **"Home Run Trades":** Slot 3 becomes *Maximize Win/Loss Ratio*. Forces the GA to hunt for asymmetric payouts even if the overall win rate drops.

### 4. Complexity Cap (The Sketch Constraint)
* **User selects:** "Human Readable Strategies Only" vs "Allow Complex Logic".
* **Under the hood:** This filters the `SketchLibrary`. If human-readable is selected, the GA is restricted to base sketches (e.g., `trend_cross`) and a maximum of 2 indicators. If complex is allowed, the GA can use deep `confluence` composite sketches.

### 5. Visual Data Selection (The Custom Sandbox)
* **User action:** The user highlights specific contiguous or non-contiguous chunks of historical data on an interactive chart (e.g., highlighting 2020 and 2022, but skipping 2021).
* **Under the hood:** This completely bypasses the need for complex algorithmic regime detection (like Hidden Markov Models). The backend stitches the user's highlighted data into a "Master Dataset". CPCV is then applied *over* this custom selection, automatically slicing it into $N$ chunks and calculating variance across them. This allows the user to intuitively build strategies targeted at very specific environments (e.g., "Crash Protection Bots") while CPCV guarantees the strategy is mathematically robust within the user's chosen world.

By exposing only these options, the user feels entirely in control of the strategy generation, but the system architect maintains total control over the mathematical integrity of the pipeline.

---

## 9. Hard Constraints (The Death Penalties)
To prevent the GA from generating "Broken Clock" strategies (e.g., a signal that is active 100% of the time to capture Beta rather than Alpha), the matrix math applies Hard Constraints *before* the Pareto sort. Any strategy violating these limits is instantly killed (assigned `-9999` fitness).

These are non-negotiable architectural guardrails:

### A. Minimum Event Count (Statistical Significance)
* **The Rule:** A strategy must fire a minimum number of discrete signals (e.g., > 50 over 3 years).
* **Why:** A strategy that yields 500% return on only 3 trades is purely curve-fit to random market anomalies.

### B. Maximum Exposure (The Beta Trap)
* **The Rule:** A strategy cannot be actively in the market for more than `X%` of the total dataset (e.g., > 40%).
* **Why:** If the strategy is "Long" on 95% of the bars, it is just tracking the underlying asset's Beta. Alpha requires market timing.

### C. Directional Symmetry (For Market Neutrality)
* **The Rule:** If running in "Long/Short" mode, the strategy's signals cannot be heavily skewed (e.g., 90% Long / 10% Short).
* **Why:** In a bull market, GAs are lazy and will simply spam "Long" signals. This constraint forces the GA to discover bidirectional edge.

### D. The "Zero Variance" Signal Check
* **The Rule:** If the strategy's signal array is uniformly `[1, 1, 1...]` or `[0, 0, 0...]`.
* **Why:** Instantly catches broken AST logic (e.g., mutating into `Close > 0` or `1 == 1`) without wasting computation time on virtual evaluations.
