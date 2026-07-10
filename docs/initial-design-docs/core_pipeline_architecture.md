# Core Pipeline Architecture

> **Context:** This document outlines the high-level, end-to-end pipeline architecture for strategy generation and validation in the greenfield system. It establishes a strict "Two-Phase" separation of concerns, separating Alpha Discovery from Execution Simulation.

---

## The Philosophy: Separation of Concerns

A core flaw in many algorithmic trading systems is mixing signal generation with risk management during the optimization phase. If a strategy fails, the system cannot distinguish whether the entry logic was inherently flawed, or if a trailing stop was simply 1 pip too tight.

To solve this, the greenfield architecture strictly divides the pipeline into two distinct phases. Phase 1 proves the mathematical edge exists. Phase 2 proves the edge can be profitably executed in the real world.

---

## Phase 1: The "Alpha Foundry"

**Goal:** Find true predictive edge across millions of genomes.
**Mechanics:** 100% Polars Vectorized Operations (CPU & GPU Ready)

In this phase, the Genetic Algorithm evaluates millions of structural sketches (indicators and parameters). There is **no stateful portfolio tracking and no sequential tick-by-tick looping**. The system evaluates entries using pre-computed Virtual TP/SL targets, purely asking: *"When this signal fires, what is the probability it hits a +2R profit before a -1R loss?"*

### 1. Polars Vectorized Evaluation (GPU-Ready)
Because there is no stateful tracking, evaluating the population is purely a structural data manipulation problem. By building Phase 1 using **100% Polars Native Expressions** (`Expr`), the system evaluates thousands of strategies simultaneously without slow row-by-row loops. 
* **Stability & Code Gen:** Using Polars instead of raw `ndarray` matrix algebra eliminates `unsafe` Rust memory conversions, guaranteeing system stability and making the codebase highly predictable for AI-assisted development.
* **GPU Acceleration:** Because Polars supports cuDF integration, using standard Polars expressions means the entire Phase 1 pipeline can automatically push execution to an NVIDIA GPU without writing custom CUDA kernels, unlocking extreme scaling for massive populations.

### 2. Alpha Target Engineering
To prevent the GA from finding "Broken Clock" strategies, the target math relies on Virtual Execution rather than raw forward returns:
* **Virtual TP/SL:** We pre-compute a Virtual Take-Profit / Stop-Loss array based on ATR (e.g., +2 ATR target, -1 ATR stop).
* **Continuous Cumulative Return:** The GA optimizes for the continuous equity curve of those virtual targets (avoiding step-function gradient failure).
* **Hard Constraints (Death Penalties):** Any strategy that fires too rarely or is exposed to the market >40% of the time is instantly killed (fitness `-9999`) before sorting.

### 3. Anti-Curve-Fitting: Combinatorial Purged Cross-Validation (CPCV)
Because GAs are ultimate curve-fitting machines, Phase 1 front-loads strict academic audits into the vectorized math:
* **CPCV Slicing:** Evaluates the strategy across multiple chronological slices simultaneously to ensure the edge persists across different macro-regimes. By slicing the Polars DataFrames, we can take the variance of performance across different market conditions with almost zero overhead.
*(Note: Slower/more pessimistic De Prado methods like DSR and PBO are reserved for the final Tear Sheet reporting in Phase 2 rather than generation constraints).*

**Phase 1 Output:** The entire **Rank 0 Pareto Frontier** (which can be thousands of unique, non-dominated strategies) is passed to Phase 2. The exact number of strategies passed is dynamic and configurable (e.g., passing 5,000 elites only takes Phase 2 a few seconds to process).

---

## Phase 2: The "Execution Simulator"

**Goal:** Validate how the Phase 1 elites survive real-world trading frictions.
**Mechanics:** Stateful Portfolio Loop (Scalar, realistic sequential simulation)

Phase 2 takes the strategies that have a proven mathematical edge and subjects them to the realities of market execution. Because Phase 2 only runs on the elite survivors (e.g., 5,000 strategies instead of 5,000,000), it can afford to use standard, slower scalar loops without causing a bottleneck.

### 1. Stateful Execution
This phase introduces:
* Hard Stop-Losses and Trailing Stops
* Take-Profit levels
* Bid/Ask Spread and Slippage
* Position Sizing and Pyramiding rules

### 2. Walk-Forward Optimization (WFO)
Phase 2 evaluates the strategies using realistic sequential deployment. 
* *Train Year 1, Trade Year 2. Train Year 1+2, Trade Year 3.*
WFO ensures that the risk management parameters (e.g., optimizing the trailing stop distance) are not overfit, and that the strategy survives sequential, out-of-sample deployment.

### 3. Standard Trading Metrics
Phase 2 calculates the metrics that traders and allocators actually care about for discrete trading:
* Walk-Forward CAGR
* Maximum Drawdown
* Win Rate / Expectancy
* Profit Factor

### 4. Safe Configuration Abstractions (Retail UI)
Because Phase 1 handles the complex math of finding Alpha, Phase 2 controls are purely focused on **Execution and Risk Management**. This safely separates the user's "Trading Goals" from the engine's "Alpha Generation".

* **Position Sizing:** "How aggressive do you want to size your trades?"
  * *Conservative:* Fixed 1% risk per trade.
  * *Smart:* Volatility-adjusted sizing (scales inversely with ATR).
* **Exit Strategy:** "How do you want to manage winning trades?"
  * *Set & Forget:* Hard Take-Profit (e.g., 2R).
  * *Let Winners Run:* ATR-based Trailing Stop.
* **Real-World Frictions:** "What market are you trading?"
  * *Low Friction (Forex/Indices):* Minimal slippage.
  * *High Friction (Crypto/Pennies):* Heavy slippage and commissions applied (quickly kills HFT strategies that passed Phase 1).
* **The Approval Hurdle:** "What is your pain threshold?"
  * E.g., Reject any strategy that suffers > 15% drawdown during Out-Of-Sample Walk-Forward testing.

---

## Pipeline Flow Dynamics (Connecting the Phases)

Rather than running strictly linearly (waiting for Phase 1 to finish completely before starting Phase 2), the greenfield system utilizes an asynchronous, high-performance flow to connect the phases. This maximizes CPU utilization and creates a real-time user experience.

### Asynchronous Producer-Consumer Architecture (The Baseline)
The two phases are entirely decoupled using multithreading and memory channels.
* **Thread A (The Producer):** Phase 1 runs constantly on a dedicated core (or the GPU), maximizing the vectorized matrix math. Every time a generation completes, any strategy that hits Rank 0 on the Pareto frontier is instantly pushed into an asynchronous queue (`crossbeam_channel`).
* **Thread Pool B (The Consumers):** A pool of worker threads constantly listens to the queue. As soon as a Phase 1 elite drops into the queue, a worker thread immediately grabs it and runs it through Phase 2's stateful execution. 
* **The Result:** Zero blocking. Phase 1 never pauses to wait for Phase 2. The user sees strategies streaming into their UI in real-time as they survive Phase 2, while the GA continues evolving in the background.

### The Ultimate Stack: Async Consumers + Nested Co-Evolution
These two approaches are not mutually exclusive; they are designed to stack perfectly. 
* **The Synergy:** Thread A (The Producer) screams ahead at 10M evals/sec, pushing elite entry rules into the queue. The Consumer Threads grab those entry rules and, rather than running a static backtest, they spin up a **Secondary Miniature GA (Nested Co-Evolution)**. 
* **The Result:** Each worker thread spends a few seconds genetically evolving the perfect bespoke Trailing Stop and Take-Profit specifically for that one entry rule. Because this happens asynchronously on worker threads, Phase 1 never has to wait. The system extracts the absolute maximum edge possible while maintaining 100% CPU utilization.

---

## The Final Output: The Strategy Tear Sheet

When a strategy survives both phases, it generates a comprehensive "Tear Sheet" that combines the pragmatic results of Phase 2 with the rigorous academic audit of Phase 1.

### Trading Performance (From Phase 2 WFO)
* **Walk-Forward CAGR:** 22.4%
* **Max Drawdown:** 8.1%
* **Win Rate:** 54.2%
* **Profit Factor:** 1.65

### Alpha Integrity Audit (From Phase 1 Array Math)
* **Deflated Sharpe Ratio:** 1.84 *(Passed: Accounts for 5 million GA iterations)*
* **CPCV Variance:** Low *(Edge is stable across all macro-regimes tested)*
* **Prob of Overfitting (PBO):** 2.1% *(Highly unlikely to be curve-fit)*

### Conclusion
By separating Alpha Generation (Phase 1) from Execution Simulation (Phase 2), the system achieves extreme computational speed, eliminates curve-fitting, and provides highly realistic performance expectations for live deployment.
