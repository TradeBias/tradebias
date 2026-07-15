# Strategy Robustness Filters & Constraints

This document outlines the Hard Constraints and Robustness Filters utilized in the Alpha Foundry to prevent data-mining bias, curve-fitting, and fragile strategies.

## 1. Active (Already Implemented) Filters

These constraints are currently coded into the engine and active in the Alpha Foundry UI.

### Transaction Cost (Slippage & Commission)
* **What it does:** Subtracts a fixed point/dollar penalty (e.g., `0.0010`) from the Gross PnL of **every single trade** evaluated by the engine.
* **Why it's critical:** In a frictionless simulation, a Genetic Algorithm will always gravitate towards High-Frequency Trading (HFT) strategies (e.g., making 10,000 trades for $1 profit each) because it creates a mathematically smooth equity curve. By baking costs directly into the fitness calculation, these fragile microscopic edges are destroyed during evolution, forcing the GA to find higher-conviction setups.

> **[opus]** This is arguably the single most important filter in the entire system. Without it, every other filter is working against a fundamentally dishonest simulation. One concern: using a fixed point value means this filter is asset-dependent. A `0.0010` penalty is enormous on a $1.00 forex pair but negligible on a $5,000 index. Consider whether a percentage-based option (e.g., `0.01%` of entry price) would be more portable across asset classes in the future. For now, fixed points is fine since we're working with one asset at a time.

### Min Trade Count
* **What it does:** Rejects any strategy that triggers fewer than `N` trades over the historical dataset.
* **Why it's critical:** A strategy that makes $10,000 across 3 trades is not a strategy; it's statistical noise (luck). Enforcing a minimum trade count ensures that the mathematical edge is statistically significant and recurring.

> **[opus]** Agree this is essential. However, the default of `10` is dangerously low. With 10 trades you have essentially zero statistical power—you cannot distinguish skill from luck. A bare minimum of **30** trades is where the Central Limit Theorem starts to give you any meaningful confidence. I would recommend a default of **50** and a hard floor of **30** that the user cannot go below. Anything less and you are presenting noise as signal.

### Occam's Penalty (Complexity Cap)
* **What it does:** Deducts a percentage from the final Fitness Score for every additional logical node (indicator or rule) added to the strategy's Abstract Syntax Tree (AST).
* **Why it's critical:** Given enough rules, you can perfectly predict the past. Occam's Razor states that the simplest explanation is usually the correct one. Punishing complexity prevents the GA from building 12-indicator spaghetti monsters that are perfectly curve-fit to historical data but will immediately fail in live trading.

> **[opus]** This is the right idea but the implementation deserves scrutiny. A flat percentage penalty per node treats all complexity equally—but a 2-node strategy using `RSI > 70 AND Close > SMA(200)` is fundamentally different from a 2-node strategy using `CRSI(3,2,100) > BB%B(20,2)`. The second one has far more implicit degrees of freedom (parameters that were optimized). Consider whether the penalty should scale with the **total parameter count** across all indicators, not just the node count. That said, node count is a reasonable proxy for Phase 1 and much simpler to implement.

### Dumb Luck Filter (Random Benchmark Percentile)
* **What it does:** Ensures the generated strategy ranks higher than a specified percentile (e.g., the 95th percentile) of completely randomized, "monkey-trading" paths.
* **Why it's critical:** If the market went straight up for 5 years, a monkey buying randomly would also look like a genius. This filter ensures the strategy is actually demonstrating true *Alpha* rather than just randomly stumbling into the market's underlying *Beta*.

> **[opus]** Excellent filter. This is effectively a poor man's White's Reality Check / Hansen's SPA Test, which is the gold standard in academic finance for testing data-mined strategies. One critical question: how are the random paths generated? If they are purely random coin-flip entries with the same holding period, that's a solid baseline. If they are random selections from the existing condition grid, that's even better because it controls for the specific market regime. The 95th percentile default is appropriate—aggressive enough to filter junk, not so aggressive that it kills everything.

---

## 2. Proposed (Recommended) Filters

These are highly recommended additional filters to implement in order to guarantee absolute safety before taking a strategy to the Phase 2 Simulator.

### Minimum Average Trade (The Ultimate Anti-HFT Filter)
* **What it does:** Rejects any strategy where the `Avg Trade` (Expected Value per trade) is less than a specific threshold (e.g., `Avg Trade < 0.0050`). 
* **Why you need it:** Even with a slippage penalty applied, some strategies might barely scrape by. Enforcing a hard minimum on the Expected Value guarantees that the strategy has a large enough "cushion" to survive unexpected real-world slippage spikes, wider spreads during news events, or broker latency.

> **[opus]** This is redundant if Transaction Cost is properly calibrated. If you set your transaction cost to `0.0010` and the GA still finds strategies with `Avg Trade = 0.0020`, that means the strategy genuinely has a `0.0030` gross edge minus `0.0010` cost = `0.0020` net. That's a real edge. Adding a Min Average Trade filter on top of that is double-punishing. I would **include it but make it optional** and default it to `0` (disabled). It becomes useful only when the user is unsure about their true transaction costs and wants an extra safety margin. Don't make it a hard requirement.

### Maximum Market Exposure (The "Anti-Beta" Filter)
* **What it does:** Rejects any strategy that is in the market more than a certain percentage of the time (e.g., `Exposure > 30%`).
* **Why you need it:** If the GA finds a strategy that is Long `90%` of the time during a massive bull market, that isn't a trading strategy—it's just a proxy for Buy & Hold. Capping exposure forces the algorithm to act like a sniper, finding surgical entries rather than just riding the macroeconomic wave.

> **[opus]** Agree this should exist, but `30%` as a default is far too aggressive. Many legitimate swing trading strategies will be in the market 40-60% of the time. Forcing exposure below 30% biases the system toward infrequent, high-conviction setups which sounds good in theory but dramatically reduces the number of trades—potentially conflicting with your Min Trade Count filter. I'd recommend a default of **50-60%** and let the user tighten it if they want. Also note: this filter already exists in the config as `max_exposure` with a default of `0.8` (80%). The question is whether the current default is too loose rather than whether we need a new filter.

### Minimum Profit Factor (Margin of Safety)
* **What it does:** Profit Factor is `Gross Profit / Gross Loss`. This filter rejects any strategy with a Profit Factor `< 1.4` (or your chosen threshold).
* **Why you need it:** A strategy with a Profit Factor of `1.1` is technically mathematically profitable, but it is incredibly fragile. One or two unexpected massive losses will immediately push it below `1.0` (unprofitable). Enforcing a minimum Profit Factor guarantees a "margin of safety" for your equity curve.

> **[opus]** I would **not** implement this as a hard GA-level filter. Profit Factor is heavily correlated with win rate and reward-to-risk ratio—it's a derivative metric, not an independent one. If you filter on Profit Factor AND Win Rate AND Average Trade, you are over-constraining the search space with redundant criteria, which will cause the GA to converge on a very narrow family of strategies (low diversity). Instead, Profit Factor belongs as a **post-hoc display filter** on the leaderboard—let the user sort/filter the results table by it, but don't kill strategies during evolution because of it.

### Minimum R-Squared (Equity Curve Linearity)
* **What it does:** Uses the `Corr Coef` metric (which measures how closely the equity curve resembles a perfectly straight 45-degree line). Rejects strategies where this is `< 0.85`.
* **Why you need it:** A strategy might make 20 points in profit over a year, but if it made 19 points in one single lucky trade in January and was completely flat for the next 11 months, it's virtually untradable. High correlation ensures the edge is consistently distributed across time and different market regimes.

> **[opus]** This is actually one of the **most powerful** filters on this list, and I would prioritize it above Profit Factor and Min Average Trade. A high R-squared equity curve is the hallmark of a genuinely robust strategy. However, `0.85` is extremely strict. Most real-world tradeable strategies have R-squared values between `0.70` and `0.90`. I'd recommend a default of **0.80** and letting the user tighten it. Also, a technical note: make sure `Corr Coef` is computed against the trade index (1, 2, 3, ..., N) not the bar index, otherwise strategies with low exposure will be penalized unfairly because of long flat stretches between trades.

---

## 3. Additional Filters to Consider

> **[opus]** The following filters are not listed above but I believe they address failure modes that the current set does not cover.

### Maximum Consecutive Losses
* **What it does:** Rejects any strategy that experienced more than `N` consecutive losing trades in its backtest history (e.g., `Max Consecutive Losses > 12`).
* **Why it matters:** This is a **psychological survivability** filter, not a mathematical one. A strategy might have a beautiful equity curve with a 60% win rate and high Sharpe, but if it had a streak of 15 consecutive losses somewhere in the data, no human being will continue trading it. They will abandon it at loss #10, miss the recovery, and declare the system broken. This filter ensures the strategy is psychologically tradeable, not just mathematically profitable.

> **[opus]** I consider this **more important than Profit Factor or Min Average Trade**. The number one reason retail traders fail is not bad strategies—it's abandoning good strategies during inevitable drawdown streaks. We already compute `max_consecutive_losses` in the metrics. Surfacing it as a hard filter is trivial and extremely high value.

### Minimum Reward-to-Risk Ratio (Avg Win / Avg Loss)
* **What it does:** Rejects strategies where the ratio of `Avg Win / Avg Loss` is below a threshold (e.g., `< 1.0`).
* **Why it matters:** A strategy can be profitable with a low reward-to-risk ratio if the win rate is high enough, but such strategies are inherently fragile. One fat-tail loss event (a flash crash, a gap through your stop) will wipe out dozens of small wins. Strategies with a reward-to-risk ratio above `1.0` are structurally more resilient to black swan events because each win can absorb more than one loss.

> **[opus]** This is a better standalone filter than Profit Factor because it directly measures the shape of the P&L distribution rather than just the aggregate ratio. That said, like Profit Factor, it should probably be a **post-hoc leaderboard filter** rather than a hard GA constraint. The GA should be free to explore low-RR / high-win-rate strategies—just let the user filter them out on the results table if that's not their preference.
