# Recommended Metrics Suite — Final Assessment

## Design Philosophy

The #1 mistake retail algo tools make is **drowning users in metrics they don't understand**, or worse, metrics that make bad strategies look good. The goal is:

1. **Headline metrics** that give an instant read on strategy quality
2. **Risk metrics** that protect the user from themselves
3. **Robustness metrics** that detect overfitting — the silent killer of retail algo traders
4. **Nothing that's just there to look impressive**

Every metric below earns its place. If it doesn't change a user's decision, it doesn't belong.

---

## UI/UX Note: Tooltip Explainers

Instead of complex visual systems (like traffic lights), **every metric in the UI will have a hover tooltip**. These tooltips will provide a short, plain-language explainer of what the metric means and, crucially, **what to look for** (e.g., "A value > 1.5 indicates a strong edge"). This ensures beginners can understand professional metrics without cluttering the interface.

---

## Phase 1: Bitwise Engine (`StrategyResult`)

These are computed inside the GA hot loop. They serve two purposes: **fitness selection** and **quick triage** when the user eyeballs candidates in the UI.

### Core — The "Glance" Metrics
*A user should be able to assess a strategy in 2 seconds from these.*

| Metric | Why It Earns Its Place |
|---|---|
| `total_trades` | Statistical validity gate. A strategy with 12 trades is noise. Users need to see this front and center to avoid false confidence. |
| `win_rate` | Universal understanding. Every trader from beginner to pro knows this. But more importantly, it's a sanity check — a 95% win rate with a 0.1 reward:risk is a ticking bomb, and pairing it with `ratio_wl` reveals that. |
| `total_pnl` | The bottom line. Did it make money? |
| `avg_trade` | **More important than total PnL.** A strategy with $50k PnL from 10,000 trades averaging $5/trade is useless after costs. This is the real edge-per-trade number. |

> **[Opus]** ✅ All four are trivially derived from running accumulators. No concerns. However, `exposure_pct` (total_trades / total_bars) is conspicuously absent from this list. It's already a column header in the leaderboard UI. A strategy in the market 95% of bars with a 1.3 PF is far worse than one in 5% with the same PF. **Recommend adding `exposure_pct` to Core.**

### Profitability — The "Is It Real?" Metrics
*Separates genuine edge from lucky streaks.*

| Metric | Why It Earns Its Place |
|---|---|
| `profit_factor` | Gross profit ÷ gross loss. The single cleanest measure of whether a strategy has systematic edge. PF > 1.5 = interesting. PF < 1.2 = probably noise after costs. Beginners grasp this easily. |
| `expectancy` | `(win_rate × avg_win) − ((1 − win_rate) × avg_loss)`. Expected $ per trade. This is the metric that teaches beginners that win rate alone means nothing. |
| `ratio_wl` | Avg win ÷ avg loss. The reward-to-risk ratio. When displayed next to win rate, it instantly tells the user what "type" of strategy they have. |
| `avg_win` | Needed to compute expectancy, but also useful standalone. |
| `avg_loss` | Same. And it's the number that tells a beginner "can I stomach this per-trade loss?" |
| `std_win` | Standard deviation of winning trades. Tells the user if the `avg_win` is reliable or skewed by one lucky outlier. |
| `std_loss` | Standard deviation of losing trades. High variance here means occasional catastrophic trades are hiding behind a benign average. |

> **[Opus]** ⚠️ `std_win` and `std_loss` require separate sum-of-squares accumulators (`sum_win_ret2`, `sum_loss_ret2`) in the hot loop. Two extra multiply-adds per trade — negligible cost, but they are **not currently implemented**. Also: `expectancy` is mathematically identical to `avg_trade`. The formula `(WR × avg_win) - ((1-WR) × avg_loss)` simplifies to `total_pnl / total_trades`. Keeping both is fine for pedagogical value (teaches beginners about edge composition), but be aware they will always show the same number.

### Risk — The "Can I Survive This?" Metrics
*These prevent account blowups.*

| Metric | Why It Earns Its Place |
|---|---|
| `max_drawdown` | **The most important risk metric in retail trading.** Most beginners quit during drawdowns. |
| `max_consecutive_losses` | Psychological preparedness. A strategy might have great stats but 15 consecutive losses will make any human override it. |
| `largest_win` | Inverse outlier detection. If one trade accounts for 80% of gross profit, the "edge" might be one lucky event. |
| `largest_loss` | Outlier detection. If one trade accounts for 60% of gross loss, the strategy might have a tail risk problem. |

> **[Opus]** ⚠️ `max_consecutive_losses`, `largest_win`, and `largest_loss` are **not currently tracked** in the accumulator. All three are trivially addable (one comparison per trade, zero measurable cost). `max_consecutive_losses` needs a running streak counter. These are the highest-ROI missing metrics — 5 lines of code each, massive UX value. **Priority: implement these first.**

### Robustness — The "Is It Overfit?" Metrics
*These are what separate TradeBias from toy backtesting tools.*

| Metric | Why It Earns Its Place |
|---|---|
| `sharpe` | Industry standard risk-adjusted return. Allows apples-to-apples comparison between strategies. |
| `sortino` | **Better than Sharpe for trading.** Only penalizes downside volatility. |
| `corr_coef` | Equity curve straightness (Pearson R). **This is your secret weapon against overfitting.** An overfit strategy has a jagged, lucky equity curve. A robust strategy has R > 0.95. |
| `cpc_index` | `win_rate × ratio_wl × profit_factor`. A composite score that's hard to game. Great as a default fitness function. |
| `t_test` | Statistical significance of the edge. |

> **[Opus]** ⚠️ `t_test` is mathematically `sharpe × sqrt(N)`. It is therefore 100% redundant with Sharpe + total_trades. A retail user looking at a Sharpe of 1.8 with 500 trades doesn't gain anything by also seeing t=40.2. **Recommend hiding behind an "Advanced" toggle or cutting entirely.** The cognitive load cost outweighs the marginal information gain. Keep it computed internally if you ever want to auto-flag statistical insignificance (e.g., greying out strategies with t < 2.0).

### Growth — The "Was It Worth It?" Metrics

| Metric | Why It Earns Its Place |
|---|---|
| `cagr` | Pass bar count + bar period to `finalize()`. Lets users compare to "just holding SPY". |
| `max_dd_duration_bars` | "How long was I underwater?" is the question every trader asks. |
| `pnl_over_dd` | The simple "was the pain worth the gain?" ratio. |

> **[Opus]** ⚠️ `cagr` is currently a placeholder (`total_pnl / 100.0`). Real CAGR requires knowing total elapsed time in years, which requires bar timeframe metadata the bitwise engine doesn't currently carry. **Recommend deferring real CAGR to Phase 2** or replacing it with a simple `return_pct` for now.
>
> ⚠️ `max_dd_duration_bars` has a subtle architectural quirk: the bitwise accumulator only visits bars where bit=1 (trade active), not every bar in the dataset. So "duration" would be measured in trade-events, not calendar bars. This is still useful but should be noted in the tooltip. True calendar-bar duration requires iterating all bars, which is feasible but changes the hot-loop structure.

### Total Phase 1 Count: **21 metrics**

---

## Phase 2: Simulator TearSheet

These are computed once per elite strategy on the full backtest. Cost doesn't matter here — thoroughness does.

**Keep everything from Phase 1**, plus add:

| Metric | Why |
|---|---|
| `initial_balance` | Context for all other numbers. |
| `final_balance` | The final account size. |
| `return_pct` | Total return as %. More intuitive than raw PnL. |
| `total_long_trades` / `total_short_trades` | Meaningful here because the simulator runs symmetric strategies. Shows directional bias. |
| `win_rate_longs` / `win_rate_shorts` | Does the strategy work equally well in both directions? |
| `avg_trade_duration_bars` | Position management insight. Affects opportunity cost and psychological comfort. |
| `average_drawdown_pct` | More realistic than max DD. |
| `avg_drawdown_duration_bars` | How long are drawdowns typically? |
| `total_costs` | If you add spread/commission modeling. Shows impact of execution costs on edge. |
| `equity_curve` | The single most important visual. |

### Total Phase 2 Additions: **10 on top of Phase 1**

---

## What to Cut (and Why)

| Metric | Verdict | Reasoning |
|---|---|---|
| `skewness_of_returns` | ❌ Cut | Academic noise. The same information is captured more intuitively by `largest_win` vs `largest_loss` and `ratio_wl`. |
| `kurtosis` | ❌ Cut | Same. "Fat tails" matters, but `largest_win`, `largest_loss`, and `max_consecutive_losses` tell the same story in plain language. |
| `r_squared` | ❌ Cut | Literally `corr_coef²`. Redundant. |
| `regression_slope` | ❌ Cut | Repackaged `avg_trade`. |
| `annual_volatility_pct` | ❌ Cut | Already embedded in Sharpe and Sortino. |
| `VaR` / `CVaR` | ❌ Cut | Institutional risk metric. Retail traders don't think in "95th percentile daily loss". Max drawdown is the retail equivalent. |
| `avg_pnl_longs` / `avg_shorts` | ❌ Cut | Redundant with `avg_trade` when you already show long/short win rates and trade counts. |
| `max_consecutive_wins` | ❌ Cut | Nobody makes bad decisions from winning streaks. |
| `SQN` / `SQM` | ❌ Cut | Same underlying math as `t_test`. |
| `K-Ratio` | ❌ Cut | Captured by `corr_coef` + `t_test`. |
| `Robust Index` | ❌ Cut | Served by `cpc_index`. |
| `E-Ratio` | ❌ Cut | Requires MFE/MAE (infeasible in bitwise). |
| `Perfect Profit %` | ❌ Cut | High architectural complexity for minimal decision value. |

> **[Opus]** ✅ Agree with every single cut. Particular emphasis:
> - `E-Ratio` is correctly flagged as infeasible. MFE/MAE requires scanning every bar *during* a trade's life to find peak unrealised profit/loss. The bitwise engine discards intra-trade paths entirely — each bar is a single pre-computed scalar PnL. Reconstructing the path would require a full bar-by-bar replay, defeating the architecture.
> - `VaR`/`CVaR` are correctly cut for Phase 1, but **consider adding `CVaR` to Phase 2's TearSheet**. In Phase 2 you have the full trade list, so sorting and averaging the worst 5% is trivial. Tooltip: "Average loss during your worst 5% of trades." More actionable than VaR for retail.
> - `max_consecutive_wins` cut is correct. Nobody abandons a strategy because it won too many times in a row.
>
> **[Opus] Missing from the cut list — should also be explicitly cut:**
> - `Calmar Ratio` (CAGR / Max DD) — redundant with `pnl_over_dd` which already captures the same concept without requiring real CAGR.
> - `Ulcer Index` — stateful drawdown-duration metric that adds complexity without changing decisions beyond what `max_dd_duration_bars` already provides.
> - `Omega Ratio` — requires the full return distribution. Overkill for Phase 1, and Sortino already captures downside-adjusted performance.

---

## Final Recommended Suite

### Phase 1 — Bitwise `StrategyResult` (21 fields)

```
Core:           total_trades, win_rate, total_pnl, avg_trade
Profitability:  profit_factor, expectancy, ratio_wl, avg_win, avg_loss, std_win, std_loss
Risk:           max_drawdown, max_dd_duration_bars, max_consecutive_losses,
                largest_win, largest_loss, pnl_over_dd
Robustness:     sharpe, sortino, corr_coef, cpc_index, t_test
Growth:         cagr
```

### Phase 2 — Simulator `TearSheet` (adds 10)

```
All of Phase 1, plus:
Balance:        initial_balance, final_balance, return_pct
Direction:      total_long_trades, total_short_trades,
                win_rate_longs, win_rate_shorts
Duration:       avg_trade_duration_bars
Drawdown:       average_drawdown_pct, avg_drawdown_duration_bars
Costs:          total_costs (when cost modeling is added)
Visual:         equity_curve
```
