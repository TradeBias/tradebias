# Phase 1: Alpha Foundry Pipeline

The Phase 1 pipeline, the "Alpha Foundry," is the core generative engine of TradeBias. Its purpose is to rapidly discover, evaluate, and evolve millions of potential trading strategies to find robust edge over a short holding period.

## 1. Core Architecture

### 1.1 The Virtual Target (5-Bar Forward Return)
Phase 1 does **not** simulate full portfolio management, stop losses, or take profits. This would require expensive row-by-row state tracking, drastically reducing evaluation speed.
Instead, it utilizes a **Virtual Target**:
1. We project exactly 5 bars into the future.
2. We calculate the price delta: `(future_close - close) / close`.
3. We assign a ternary state to every bar:
   - `+1.0`: The price moved up (Win for Long, Loss for Short).
   - `-1.0`: The price moved down (Loss for Long, Win for Short).
   - `0.0`: The price remained perfectly flat (Neutral).

### 1.2 AST to Polars Compilation
Strategies are internally represented as an Abstract Syntax Tree (AST), defined by the `tb_core::Sketch` struct.
To achieve massive parallelization, we compile these ASTs directly into **Polars Expressions**. 
For example, an AST representing `RSI(14) > 70` is compiled into `polars::lazy::dsl::col("rsi_14").gt(70.0)`.

Once evaluated, this outputs a Boolean mask for the entire historical dataset indicating whether the strategy's entry condition is met on each bar.

### 1.3 Matrix Dot Product Evaluation
We cast the Boolean entry mask to floats:
- Long Strategy: `1.0` when active, `0.0` otherwise.
- Short Strategy: `-1.0` when active, `0.0` otherwise.

We then perform an element-wise dot product between the Strategy Signal vector and the Virtual Target vector.
`strategy_return = signal_vector * virtual_target_vector`

The sum of this resulting vector represents the raw, unadjusted fitness of the strategy. A positive sum means the strategy caught more beneficial 5-bar movements than detrimental ones.

## 2. Genetic Algorithm (GA) & Evolution

### 2.1 The Population
The engine maintains a population of `Sketch` ASTs. At Gen 0, this population is seeded using predefined structural templates (e.g., Mean Reversion, Trend Following, Breakout). 

### 2.2 Fitness Calculation & Penalties
To prevent curve-fitting and overtrading, we apply severe penalties to the raw fitness score. If a strategy's final score drops below 0.0, it is considered dead.

1. **Slippage Penalty**: High-frequency noise is heavily penalized. We calculate the number of actual trades (transitions from 0 to 1) and subtract `config.slippage_penalty` points per trade.
2. **Complexity Penalty (Occam's Razor)**: To prevent massive, convoluted, over-fitted indicator trees, we subtract `200.0` points for every indicator node present in the AST. 
3. **Hard Constraints**: Strategies are instantly killed (`fitness = -9999.0`) if they violate user-defined constraints:
   - `min_trades`: The strategy must execute enough trades to be statistically significant.
   - `min_exposure` / `max_exposure`: The strategy must not be overly stagnant or essentially buy-and-hold.

*(Note: We do NOT use a benchmark penalty / Alpha Isolation in Phase 1, as 5-bar scalps are designed to capitalize on localized volatility independent of macro-trends, and applying macro-penalties mathematically destroys directionally balanced evolution).*

### 2.3 Reproduction & Mutation
At the end of each generation:
1. We sort the survivors using a multi-objective Pareto sort.
2. We utilize **Tournament Selection** (picking 3 random strategies and choosing the best) to select parents for the next generation.
3. We perform **Type-Safe Subtree Crossover**: A random Boolean or Float node in Parent A is swapped with a node of the exact same return type from Parent B.
4. We apply **Mutation**: Indicator periods are shifted, constants are jittered, or `CrossAbove` is flipped to `CrossBelow`.

## 3. MAP-Elites Archive
To preserve genetic diversity and prevent the population from homogenizing on a single local maximum, Phase 1 employs a **MAP-Elites Archive**.

- The archive acts as an elite hall of fame.
- As strategies are evaluated, they attempt to enter the Archive.
- The Archive bins strategies based on their fundamental traits (e.g., trade frequency, complexity, regime type).
- Only the highest-fitness strategy for a specific bin is retained.
- Elites from the Archive are routinely injected back into the breeding pool (typically 10% of the population) to cross-pollinate with new generations.

## 4. Engine Variants
To maximize hardware utilization, Phase 1 supports three distinct compilation engines:

1. **Continuous AST Engine (`continuous.rs`)**: Recompiles the full AST for every strategy, every generation. Highly flexible, requires zero memory overhead, but is CPU intensive.
2. **Discrete Engine (`discrete.rs`)**: Pre-computes a massive grid of every possible indicator permutation upfront. The GA simply snaps AST nodes to the nearest pre-computed grid point. Requires massive RAM but is blazingly fast.
3. **Hybrid Engine (`hybrid.rs`)**: Uses a Just-In-Time (JIT) cache. If a generation mutates a new indicator parameter (e.g., `SMA(33)`), it computes it once and appends it to a growing Polars DataFrame cache. Future strategies reusing `SMA(33)` pull directly from RAM. Strikes the perfect balance between memory usage and speed.
