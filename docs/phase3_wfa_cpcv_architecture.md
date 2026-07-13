# TradeBias Phase 3 Architecture: WFA & CPCV

**To:** Quantitative Systems Architecture Team
**Subject:** Implementing Walk-Forward Analysis and CPCV as Core Generation Variants

## 1. The Methodological Distinction

During Phase 1 and 2 development, a critical distinction was established regarding robustness testing:

*   **Strategy-Level Validation (Static):** Tests like Feature Ablation, Noise Injection, and Trade Deletion stress-test a *fixed* logic ruleset (e.g., `RSI < 30 & SMA > 50`). These are perfect for instant post-processing popups because they evaluate a specific AST.
*   **System-Level Validation (Dynamic):** Walk-Forward Analysis (WFA) and Combinatorial Purged Cross-Validation (CPCV) are optimization workflows, not passive tests. They require the engine to constantly change the strategy logic over rolling time windows to adapt to new market regimes. 

Attempting to run WFA retroactively on a fixed strategy found during a standard In-Sample run introduces massive Look-Ahead Bias (Data Snooping), as the strategy has already "seen" the future data.

## 2. The Architectural Solution

To achieve institutional-grade validation, WFA and CPCV must be integrated into TradeBias not as post-process tests, but as **Generation Loop Variants**.

Instead of treating the Genetic Algorithm as a monolithic block that searches the entire historical dataset once, the user will select a "Validation Architecture" prior to clicking "Start Evolution".

### Proposed UI Updates (Alpha Foundry)
Replace the current "In-Sample Split" slider with a **Validation Architecture Dropdown**:
1.  **Standard IS / OOS (Phase 1):** The GA trains on a single contiguous block (e.g., first 70%) and outputs fixed AST strategies.
2.  **Walk-Forward Analysis (WFA) (Phase 3):** The user defines rolling windows (e.g., 2 Years Train / 6 Months Test). The engine runs the GA multiple sequential times.
3.  **CPCV (Phase 3):** The engine slices history into `N` blocks and runs the GA across all possible permutations to build an exact probability distribution of backtest overfitting (PBO).

## 3. The Execution Flow for WFA

If WFA is selected, the engine executes the following logic:
1.  The raw price data is partitioned into overlapping rolling windows.
2.  The engine dynamically generates a distinct `ConditionGrid` and `TargetOutcomes` tensor for *every single window*.
3.  The Genetic Algorithm is executed in a loop. For Window 1, the GA finds the best ruleset and evaluates it on Out-of-Sample 1. The GA is completely purged, and restarts for Window 2, finding a potentially completely different ruleset for Out-of-Sample 2.
4.  The output sent to the Leaderboard is no longer a single AST. It is an **Algorithmic Pipeline** (a stitched composite equity curve of the dynamically adapting engine).

## 4. Computational Feasibility

Standard retail platforms require hours to run WFA because they must recalculate indicators chronologically for every generation. 

TradeBias is uniquely positioned to execute WFA in seconds dynamically in the browser/UI. Because the Bitwise Engine relies on SIMD bitmask intersections, "re-running the GA 10 times" simply means iterating a bitwise loop against 10 precomputed `u64` vectors. The primary bottleneck will simply be the initial RAM allocation for the 10 rolling `ConditionGrid` states.

*Drafted for future Phase 3 integration.*
