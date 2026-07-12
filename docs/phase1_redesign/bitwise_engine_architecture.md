# Next-Generation Phase 1 Architecture: The Bitwise Engine

This document outlines the architectural analysis of ultra-high-speed strategy generation platforms (e.g., BuildAlpha, reaching 30,000+ strategies/sec) and details a proposed roadmap for migrating TradeBias from our current Polars-based engine to a custom Rust Bitwise Engine.

## 1. Analysis: How Competitors Achieve 30k+ Strats/Sec

When observing platforms that evaluate tens of thousands of strategies per second with **full backtest metrics** (Drawdown, Net PnL, Win Rate), the assumption that this speed comes solely from using C/C++ is incorrect. Rust possesses the exact same bare-metal execution speed. The difference is strictly architectural.

These platforms abandon traditional mathematical evaluation and stateful backtesting in favor of two primary techniques:

### A. Bitwise Masking
Instead of calculating `SMA(50) > SMA(200)` dynamically, the engine pre-computes every possible indicator condition for the entire dataset before Generation 1 begins. 
These conditions are stored not as floats or booleans, but packed into **Bitsets** (arrays of 64-bit integers). 
- Bar 0 is bit 0, Bar 1 is bit 1, etc.
- To evaluate a strategy like `Condition A AND Condition B`, the engine performs a CPU-level bitwise `&` operator. Modern CPUs (via AVX/SIMD) can evaluate 256 to 512 bars of data in a single clock cycle. This completely bypasses floating-point arithmetic.

### B. Pre-computed Trade Outcomes
To calculate full metrics without a slow, row-by-row simulation loop, the engine enforces fixed exit conditions (e.g., 2% TP, 1% SL, or 5-bar time exit). 
Because the exit parameters are fixed, the exact outcome of a trade can be **pre-calculated for every single bar in the dataset**.
- The engine creates a flat array: `Trade_PnL[N]`.
- If the bitmask for a strategy has a `1` at Bar `145`, the engine simply adds `Trade_PnL[145]` to the total profit. There is no state management, no looping over subsequent bars to check for stop-losses, and zero branching (`if/else` logic). 

## 2. Roadmap: Achieving This in TradeBias

To replicate these speeds (30,000+ strats/sec) and unlock full Phase 1 backtest metrics, we must implement the following pipeline:

1. **Completely Remove Polars:** Polars is a fantastic data science tool, but its query optimizer, expression graphs, and memory allocations introduce fatal framework overhead. We will remove it entirely from the dependency tree.
2. **Initialization Pre-computation Phase:** We parse the historical CSV using the standard Rust `csv` crate and load the data directly into raw, contiguous `Vec<f64>` arrays (Open, High, Low, Close, Volume).
3. **Custom Native Indicators:** We will NOT use third-party Rust technical analysis packages. We will write our own highly optimized, bare-metal indicator functions that operate directly over our `Vec<f64>` arrays.
4. **Bit-Packing:** We convert the generated boolean condition series (e.g., `RSI(14) < 30`) into raw `Vec<u64>` bit-arrays.
5. **Outcome Pre-computation:** We run a one-time forward scan over the dataset using the user's defined Stop Loss / Take Profit / Time exits to populate `Trade_PnL[N]`, `Trade_Duration[N]`, and `Trade_MaxDrawdown[N]` arrays.
6. **The Custom Evaluation Loop:** The genetic algorithm will assemble strategies purely as pointers to these bit-arrays.

## 3. What Will Replace Polars?

We do not need a heavy framework. We need raw, contiguous memory arrays.
- **For CSV Parsing:** We will use the standard Rust `csv` crate. While it is single-threaded, parsing a CSV is a one-time startup cost.
- **For Data Storage & Indicators:** We will use native Rust `Vec<f64>` arrays and write custom mathematical functions for all indicators (SMA, RSI, MACD, etc.).
- **For Evaluation (The Hot Loop):** We will use native Rust `Vec<u64>` bitsets.

## 4. GPU Acceleration (`wgpu`)

By dropping Polars and structuring our data as raw, contiguous memory arrays (Bitsets and PnL arrays), we perfectly align our architecture for **Zero-Transfer GPU Compute**.

Instead of relying on Polars' `cuDF` (which suffers from CPU-to-GPU transfer bottlenecks and is locked to NVIDIA), we will use the **`wgpu`** crate.
- `wgpu` allows us to write Compute Shaders (in WGSL) that run on **any GPU** (NVIDIA, AMD, Apple Silicon).
- We transfer the massive pre-computed bitsets and `Trade_PnL` arrays to the GPU VRAM exactly *once* at startup.
- **Flipping the Matrix:** During evaluation, instead of evaluating 1 strategy across 100,000 bars on the CPU, we instruct the GPU to evaluate **10,000 strategies simultaneously** in parallel threads.
- Because bitwise `AND` operations and vector summations are native to GPU architecture, evaluating an entire population takes fractions of a millisecond.

## 5. Advantages vs. Disadvantages

### Advantages
- **Blazing Execution Speed:** Easily achieve 10k to 50k strategies per second, even on standard consumer hardware.
- **Full Phase 1 Metrics:** We no longer need to rely on the simplified "Virtual Target" proxy. Strategies can be evaluated on real Net PnL, Max Drawdown, and Win Rate inside Phase 1.
- **Deterministic Memory:** Memory is allocated exactly once at startup. The GA loop requires zero new allocations, eliminating Garbage Collection / Drop overhead.

### Disadvantages
- **Loss of Dynamic Exits:** Because trade outcomes must be pre-calculated, strategies cannot have dynamic exit logic (e.g., "Exit when RSI > 70"). Exits must be strictly structural (Fixed SL/TP, Trailing Stops, or Time-based).
- **Combinatorial Explosion (RAM):** Pre-computing *every* possible indicator condition requires significant upfront RAM. If we have 10 indicators, 5 periods, and 3 parameters, the grid size grows exponentially. 
- **Loss of Expressive Power:** Polars expressions allow complex mathematical nesting (`SMA(EMA(Close)) / ATR`). In a bitwise engine, we are restricted to strictly the conditions we pre-computed at startup.
- **Code Complexity:** Bitwise operations and SIMD intrinsics are harder to debug and trace than declarative Polars queries.

## 6. Managing the Combinatorial Explosion (The Pre-Computation Filter)

If we pre-compute 10,000 base conditions, combining them into 5-condition strategies results in quadrillions of possibilities. If we allow the GA to blindly search this space, it will drown in mathematical garbage. 

To ensure the GA finds robust alpha without over-engineering templates or risking premature convergence, this architecture relies heavily on **Modular Pre-Computation Filtering**. We must clean the grid *before* the GA starts. 

The pre-computation module will enforce three critical rules:

### A. Semantic Type-Checking (Grid Pruning)
It is mathematically useless to evaluate `RSI(14) > SMA(200)` because RSI is an oscillator (0-100) and SMA is a price (e.g., $1.15). 
During the pre-computation phase, we will strictly tag indicators by type:
- **Price** crosses **Price**.
- **Oscillators** cross **Constants** or **Oscillators**.
- **Volume** crosses **Volume**.

By enforcing semantic logic, we remove the vast majority of combinations before they are even stored in memory. This drastically shrinks RAM requirements and completely removes mathematical gibberish from the GA's search space without artificially restricting its creativity.

### B. Bitwise Correlation Culling (Convergence Killer)
GAs often fail because of **premature convergence**—they find a good mechanic (`SMA50 > SMA200`) and fill the entire population with slightly altered clones (`EMA50 > EMA200`, `WMA50 > SMA200`). 

To force the GA to construct strategies out of truly unique, orthogonal concepts, we will run a fast bitwise `XOR` check across our pre-computed base conditions.
If two conditions trigger on the exact same bars 95%+ of the time, the pre-computation module will **delete one of them from memory**. 

By intentionally deleting highly correlated duplicates from the grid, the GA physically cannot converge on slight variations of the same strategy. It is forced to mate orthogonal concepts, guaranteeing massive diversity and exposing novel edge cases.

### C. Sparsity & Density Culling (Frequency Filter)
Some mathematical conditions are completely useless because of their frequency. If `RSI(2) < 5 AND MACD < -20` only triggers 3 times in 100,000 bars, it has no statistical significance. Conversely, if `Close > SMA(200)` is true 85% of the time, it provides no timing advantage.
Because counting bits (`popcnt`) takes a single CPU cycle, the pre-computation module will instantly count the `1`s in every bitset and delete any condition that triggers < 1% of the time or > 50% of the time. 

## 7. The 30k/sec Speed Hacks

Once the grid is pruned using the three rules above, the Genetic Algorithm utilizes two critical speed hacks to reach maximum evaluation throughput:

### A. The "Target Bitmask"
Calculating a strategy's Win Rate typically requires looping over a float array of trade outcomes. We skip this entirely by turning the Trade Outcomes themselves into Bitsets.
- `Winning_Trades_Mask`: Contains a `1` on every bar where taking a trade results in a profit.
- `Losing_Trades_Mask`: Contains a `1` on every bar where taking a trade results in a loss.

A strategy's Win Rate is calculated purely via silicon-level bitwise logic, requiring zero floating-point math:
`Win Rate = popcnt(Strategy_Mask & Winning_Trades_Mask) / popcnt(Strategy_Mask)`

### B. Zobrist Hashing (Symmetrical Duplicate Caching)
In a GA, `Condition A AND Condition B` is mathematically identical to `Condition B AND Condition A`.
We assign a unique 64-bit random integer (a Zobrist Hash) to every base condition. When the GA constructs a strategy, it simply `XOR`s the hashes together. We store this hash in a fast cache. If the GA ever produces a combination it has already evaluated, it instantly skips evaluation and pulls the cached fitness score, saving massive amounts of compute.

## 8. The Agentic Test Bench (Polars Deprecation)

Transitioning to a raw Bitwise Engine introduces significant mathematical complexity (bit manipulation, SIMD vectors, pre-calculated PnL arrays). To ensure that mathematical bugs (especially regarding Short strategy logic) are never introduced into the system, **the legacy Polars engine is formally deprecated** and will be replaced by an **Agentic Test Bench** (`tb_bench`).

### A. Pluggable Architecture
We will introduce an `AlphaEngine` trait. The deprecated Polars engine will be wrapped in this trait to serve strictly as a mathematical baseline. The new Bitwise engine will also implement this trait, allowing side-by-side execution.

### B. Oracle Datasets
To mathematically lock the system, the Test Bench will utilize "Oracle Datasets"—tiny, 20-bar handcrafted CSV files with deliberately engineered uptrends, downtrends, and flat periods. The expected PnL for Long and Short strategies on these oracles will be hardcoded. 
If the Bitwise engine produces a PnL that deviates from the Oracle by even a fraction of a cent, the bench will fail and explicitly alert the developer/AI to the exact bar that caused the failure.

### C. AI-Readable Profiling (The AGY Loop)
The `tb_bench` crate will output highly structured JSON/Markdown reports detailing:
- Strategies per second
- Peak RAM usage
- Number of conditions culled (Correlation/Sparsity)
- Generation convergence metrics

This provides the AI agent with an autonomous feedback loop. The AI can write a new optimization (e.g., Zobrist hashing), compile the bench, verify that the Oracle math didn't break, and read the profiling report to mathematically prove that execution speed increased before finalizing the code.

## 9. Rayon Multi-Threading (Dependency Independence)

Because the evaluation loop strictly reads from pre-calculated `Vec<u64>` bitsets without managing state, mutating data, or holding locks (Mutexes), the architecture is perfectly poised for **zero-cost data parallelism** via the `rayon` crate.
- `rayon` operates completely independently of Polars. 
- It allows the Genetic Algorithm to distribute the evaluation of 5,000 strategies across all available CPU cores by simply changing standard iterators (`.iter()`) to parallel iterators (`.par_iter()`).
- Because of perfect linear scaling, an 8-core machine running `rayon` will boost a 12k strats/sec (8 years data) single-thread execution to nearly **96k strats/sec**, permanently eliminating any computational bottlenecks without introducing heavy data-science dependencies.
