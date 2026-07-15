# Deep Analysis: Traditional Indicators vs. Mathematical Primitives

**Author:** Project Manager / Lead Quant  
**Date:** 2026-07-14  
**Status:** APPROVED FOR IMPLEMENTATION  
**Topic:** Replacing hardcoded retail indicators (RSI, MACD, etc.) with core mathematical primitives in the Alpha Foundry engine.

---

## 1. The Current State: Hardcoded Indicators

Currently, our engine relies on pre-computing ~100 traditional technical indicators (SMA, RSI, MACD, Bollinger Bands, Ehlers filters, etc.) in `tb_math` and feeding them to the Genetic Algorithm (GA) as boolean condition blocks (e.g., `RSI > 70`).

### Why This Is a Problem
1. **Accuracy Risk:** We have no guarantee that our implementations of MACD or RSI exactly match the broker's or MetaTrader's implementations. Minor differences in smoothing methods (Wilder's vs. Exponential vs. Simple) can shift entries/exits by 1-2 bars, completely altering backtest results compared to live execution.
2. **Theoretical Dead End:** In professional quantitative finance, relying on retail indicators is widely considered a dead end. They are lagging, widely understood by the market, and their default lookback periods (e.g., 14, 26, 9) are completely arbitrary.
3. **Bloated AST:** The current `Expr` enum has ~100 indicator variants, each requiring dedicated computation code, Display formatting, and serialization. This is unmaintainable.

---

## 2. The Pivot to "Primitives" (The Alpha-101 Approach)

Abandoning "named indicators" in favor of mathematical and structural primitives is how top-tier hedge funds (WorldQuant, Renaissance Technologies) build Alphas. Instead of hardcoding "RSI", we provide the GA with mathematical building blocks and let it compose its own formulas.

### Core Insight: The 5 Fundamental Market Questions

Every technical indicator in existence is really just asking one of 5 fundamental questions about price. The GA does not need to discover "RSI" or "CCI" specifically — it needs to discover that asking one of these questions has predictive value, and then compose the simplest possible formula that answers it.

| Fundamental Question | Classic Indicators That Ask It | Shallow Primitive Equivalent (Depth 2-4) |
|:---|:---|:---|
| **Is price trending?** | MACD, ADX, MA Crossovers, TRIX, KST, PPO | `Sub(Ema(Close, fast), Ema(Close, slow))` — depth 2 |
| **How volatile is price?** | ATR, Bollinger Width, Historical Vol, Chaikin Vol, Ulcer Index | `StdDev(Close, n)` — depth 1, or `Div(StdDev(Close, n), Sma(Close, n))` — depth 2 |
| **Where is price in its range?** | RSI, Stochastic, Williams %R, CCI, Z-Score, Bollinger %B, CMO | `Div(Sub(Close, Ts_Min(Low, n)), Sub(Ts_Max(High, n), Ts_Min(Low, n)))` — depth 3 |
| **Is volume confirming price?** | OBV, PVT, MFI, Chaikin Money Flow, Force Index, NVI/PVI | `Div(Ts_Sum(Mul(Volume, Sub(Close, Delay(Close, 1))), n), Ts_Sum(Volume, n))` — depth 4 |
| **What is the candle's shape?** | Doji, Hammer, Engulfing, Morning Star, Shooting Star | `Div(Abs(Sub(Close, Open)), Sub(High, Low))` — depth 3 |

### Composability Audit: Current Indicators → Required Primitives

A core requirement is that **the primitives must be able to organically compose all of the traditional indicators we currently support** (excluding acknowledged exceptions).

| Indicator / Concept | Required Primitives for Composition | Composability Status |
|:---|:---|:---|
| **MACD / PPO** | `Ema(x, n)`, `Sub`, `Div` | ✅ Fully Composable (`Ema(12) - Ema(26)`) |
| **Bollinger Bands** | `Sma(x, n)`, `StdDev(x, n)`, `Add`, `Mul`, `Constant` | ✅ Fully Composable (`Sma(20) + (StdDev(20) * Constant(2))`) |
| **Stochastic / Williams %R** | `Ts_Max`, `Ts_Min`, `Sub`, `Div` | ✅ Fully Composable (`(Close - Min) / (Max - Min)`) |
| **Donchian / Keltner** | `Ts_Max`, `Ts_Min`, `Ema`, `TrueRange`, `Add`, `Mul` | ✅ Fully Composable |
| **CCI / Z-Score** | `Sma`, `StdDev`, `Abs`, `Sub`, `Div` | ✅ Fully Composable (`(Price - Sma) / StdDev`) |
| **ATR / Supertrend** | `TrueRange`, `Rma(x, n)`, `Sma`, `Mul`, `Add` | ✅ Fully Composable |
| **HMA / DEMA / TEMA** | `Wma`, `Ema`, `Mul`, `Sub` | ✅ Fully Composable |
| **ROC / Momentum** | `Sub`, `Div`, `Delay` | ✅ Fully Composable (`(Close - Delay(Close, n)) / Delay(Close, n)`) |
| **OBV / PVT / NVI** | `Volume`, `Close`, `Delay`, `Mul`, `Ts_Sum` | ✅ Fully Composable |
| **ADX / DMI** | `TrueRange`, `Ts_Max`, `Ts_Min`, `Rma`, `Delay`, `Abs` | ✅ Fully Composable |
| **Candlestick Patterns** | `Abs`, `Sub`, `Div`, `Delay`, `Open`, `Close`, `High`, `Low` | ✅ Fully Composable (e.g. body ratio = `Abs(Close-Open) / (High-Low)`) |
| **Linear Regression** | `LinRegSlope(x, n)` | ✅ Requires dedicated statistical primitive |
| **RSI** | `Rma`, `Sub`, `Delay`, `Max`, `Div` | ⚠️ Composable but requires depth 7+ (see Section 3) |
| **Connors RSI / TSI** | Multiple nested smoothers | ⚠️ Requires depth 8+ — unreachable at constrained depths |
| **PSAR / Kalman / Ehlers** | Recursive loops, complex trig, state machines | ❌ Exception — keep as hardcoded advanced nodes |

---

## 3. Design Philosophy: Depth Constraints & Unreachable Classics

### AST Maximum Depth Limit
The random tree generator MUST enforce a **maximum AST depth of 5 nodes**. This is critical to prevent:
- Computationally expensive deeply nested trees
- Mathematically meaningless over-composition (e.g., `Ema(StdDev(Rma(Sma(Ema(Close)))))`)
- Exponential search space explosion

### Intentionally Unreachable Indicators
Some classic indicators (RSI, Connors RSI, TSI) require AST depths of 7-8+ to compose from pure primitives. At a depth cap of 5, the GA **cannot** represent the exact Welles Wilder RSI formula. **This is intentional and desirable.**

The GA does not need to rediscover RSI. RSI's specific formula — the Wilder's smoothing, the 100-minus inversion, the gain/loss split — is arbitrary (invented in 1978 with no mathematical justification for those design choices). The GA will instead discover *simpler, more direct* formulas that answer the same fundamental question ("where is price in its range?") at depth 3-4. For example:
- `Div(Sub(Close, Ts_Min(Low, 14)), Sub(Ts_Max(High, 14), Ts_Min(Low, 14)))` captures functionally equivalent momentum/mean-reversion dynamics at depth 3.
- `Div(Sub(Close, Sma(Close, 14)), StdDev(Close, 14))` (Z-Score) captures the same "distance from mean" concept at depth 3.

These shallow alternatives are arguably *better* than RSI — they are mathematically cleaner, computationally cheaper, and not burdened by decades of retail convention.

---

## 4. Minimum Required Primitives Library

To guarantee the composability outlined above, the engine must supply exactly these building blocks:

### 4a. Data Sources (Leaf Nodes)
| Primitive | Semantic Type | Description |
|:---|:---|:---|
| `Close` | `Price` | Current bar's closing price |
| `Open` | `Price` | Current bar's opening price |
| `High` | `Price` | Current bar's high price |
| `Low` | `Price` | Current bar's low price |
| `Volume` | `Volume` | Current bar's traded volume |
| `Constant(f64)` | `Scalar` | A fixed numeric value (e.g., 2.0, 0.5, 100.0) |

### 4b. Core Arithmetic (Binary Nodes)
| Primitive | Signature | Description |
|:---|:---|:---|
| `Add(x, y)` | Same Type → Same Type | Addition |
| `Sub(x, y)` | Same Type → Same Type | Subtraction |
| `Mul(x, y)` | See type algebra below | Multiplication |
| `Div(x, y)` | See type algebra below | Division |
| `Abs(x)` | T → T | Absolute value (preserves type) |

### 4c. Time-Series Aggregation
| Primitive | Signature | Description |
|:---|:---|:---|
| `Delay(x, n)` | T → T | Value of `x` from `n` bars ago |
| `Ts_Max(x, n)` | T → T | Rolling maximum over `n` bars |
| `Ts_Min(x, n)` | T → T | Rolling minimum over `n` bars |
| `Ts_Sum(x, n)` | T → T | Rolling sum over `n` bars |

### 4d. Smoothing Primitives (The Engines of Finance)
| Primitive | Signature | Description |
|:---|:---|:---|
| `Sma(x, n)` | T → T | Simple Moving Average |
| `Ema(x, n)` | T → T | Exponential Moving Average |
| `Wma(x, n)` | T → T | Weighted Moving Average |
| `Rma(x, n)` | T → T | Wilder's Smoothing (used in ATR/ADX) |

### 4e. Statistical Operators
| Primitive | Signature | Description |
|:---|:---|:---|
| `StdDev(x, n)` | T → T | Rolling Standard Deviation |
| `LinRegSlope(x, n)` | T → `Ratio` | Slope of linear regression over `n` bars |

### 4f. Market Internals
| Primitive | Signature | Description |
|:---|:---|:---|
| `TrueRange()` | → `Price` | `Max(High - Low, Abs(High - Delay(Close, 1)), Abs(Low - Delay(Close, 1)))` |

### 4g. Logical / Relational (Produce Boolean)
| Primitive | Signature | Description |
|:---|:---|:---|
| `GreaterThan(x, y)` | Same Type → `Boolean` | True if `x > y` |
| `LessThan(x, y)` | Same Type → `Boolean` | True if `x < y` |
| `CrossAbove(x, y)` | Same Type → `Boolean` | True if `x` crosses above `y` this bar |
| `CrossBelow(x, y)` | Same Type → `Boolean` | True if `x` crosses below `y` this bar |
| `And(a, b)` | `Boolean, Boolean` → `Boolean` | Logical AND |
| `Or(a, b)` | `Boolean, Boolean` → `Boolean` | Logical OR |

### 4h. Hardcoded Exception Nodes (Black Boxes)
These indicators use recursive state machines, complex trigonometry, or matrix operations that cannot be reasonably composed from primitives. They are retained as opaque leaf/function nodes:

| Node | Rationale |
|:---|:---|
| `Psar { af_step, af_max }` | Accelerating feedback loop with directional state |
| `KalmanFilter { r, q }` | Kalman state estimation (matrix math) |
| `EhlersSuperSmoother { period }` | DSP filter with trig coefficients |
| `EhlersDecycler { period }` | High-pass filter |
| `EhlersCyberCycle { alpha }` | Hilbert Transform based |

---

## 5. Semantic Type System & Type Algebra

To prevent the GA from generating mathematically meaningless formulas (e.g., `Close + Volume`), the AST generator must enforce strict dimensional analysis via a **Semantic Type System**.

### 5a. Semantic Types
```
pub enum SemanticType {
    Price,    // Absolute dollar values (Open, High, Low, Close)
    Volume,   // Shares/contracts traded
    Ratio,    // Dimensionless (percentages, oscillators, normalized values)
    Scalar,   // Constants (2.0, 0.5, 100.0) — dimensionless multipliers
    Boolean,  // True/False triggers
}
```

### 5b. Complete Type Algebra Table

The random tree generator MUST consult this table when selecting operands. If a combination is marked **FORBIDDEN**, the generator must never produce it.

| Operation | Left Type | Right Type | Result Type | Example |
|:---|:---|:---|:---|:---|
| `Add` / `Sub` | `Price` | `Price` | `Price` | `Close - Sma(Close, 20)` |
| `Add` / `Sub` | `Ratio` | `Ratio` | `Ratio` | `LinRegSlope - Constant(0.5)` |
| `Add` / `Sub` | `Volume` | `Volume` | `Volume` | `Volume - Sma(Volume, 20)` |
| `Add` / `Sub` | `Price` | `Volume` | **FORBIDDEN** | — |
| `Add` / `Sub` | `Price` | `Ratio` | **FORBIDDEN** | — |
| `Add` / `Sub` | `Volume` | `Ratio` | **FORBIDDEN** | — |
| `Mul` | `Price` | `Scalar` | `Price` | `StdDev(Close, 20) * Constant(2)` |
| `Mul` | `Volume` | `Scalar` | `Volume` | `Volume * Constant(0.5)` |
| `Mul` | `Ratio` | `Scalar` | `Ratio` | `LinRegSlope * Constant(100)` |
| `Mul` | `Scalar` | `Scalar` | `Scalar` | `Constant(2) * Constant(3)` |
| `Mul` | `Price` | `Price` | **FORBIDDEN** | Dollars² has no meaning |
| `Mul` | `Volume` | `Volume` | **FORBIDDEN** | — |
| `Mul` | `Price` | `Ratio` | `Price` | Scale a price by a ratio |
| `Mul` | `Volume` | `Ratio` | `Volume` | Scale volume by a ratio |
| `Mul` | `Price` | `Volume` | **FORBIDDEN** | — |
| `Div` | `Price` | `Price` | `Ratio` | `Close / Sma(Close, 20)` |
| `Div` | `Volume` | `Volume` | `Ratio` | `Volume / Sma(Volume, 20)` |
| `Div` | `Price` | `Scalar` | `Price` | `(High + Low) / Constant(2)` |
| `Div` | `Volume` | `Scalar` | `Volume` | — |
| `Div` | `Ratio` | `Scalar` | `Ratio` | — |
| `Div` | `Ratio` | `Ratio` | `Ratio` | — |
| `Div` | `Price` | `Volume` | **FORBIDDEN** | — |
| `Div` | `Volume` | `Price` | **FORBIDDEN** | — |
| `GreaterThan` / `LessThan` | `Price` | `Price` | `Boolean` | `Close > Sma(Close, 50)` |
| `GreaterThan` / `LessThan` | `Ratio` | `Ratio` | `Boolean` | `LinRegSlope > Constant(0)` |
| `GreaterThan` / `LessThan` | `Ratio` | `Scalar` | `Boolean` | `Z-Score > Constant(1.5)` |
| `GreaterThan` / `LessThan` | `Price` | `Volume` | **FORBIDDEN** | — |

**Note on `Scalar` vs `Ratio`:** A `Scalar` is a compile-time constant (`Constant(2.0)`). A `Ratio` is a computed dimensionless value (e.g., `Close / Sma(Close, 20)`). For the purposes of comparison operators, `Scalar` is compatible with `Ratio` (both are dimensionless). For `Add`/`Sub`, `Scalar` is also compatible with `Ratio`.

**Note on Smoothers:** Smoothing primitives (`Sma`, `Ema`, `Wma`, `Rma`) preserve the semantic type of their input. `Sma(Price, n) → Price`. `Ema(Ratio, n) → Ratio`. The same applies to `Ts_Max`, `Ts_Min`, `Ts_Sum`, `Delay`, and `Abs`.

---

## 6. Implementation Specification for AI Agents

*Note for LLMs / Coding Agents reading this blueprint: You are cleared to execute this architectural pivot based on the following context.*

### Target Environment & Constraints
*   **Workspace:** The project is a Cargo Workspace with strictly siloed crates (`tb_core`, `tb_math`, `tb_bitwise`, `tb_simulator`, `tb_ui`).
*   **Zero-Lifetime Policy:** DO NOT USE RUST LIFETIMES (`<'a>`). Rely strictly on owned types and `Arc`. The engine is highly parallelized.
*   **Data Pipelines:** We use `Polars` for fast initial series calculation, and primitive flat arrays `Vec<f64>` / `Vec<bool>` in `tb_bitwise` for matrix caching.
*   **AST Max Depth:** Enforce a hard maximum depth of **5** on all generated expression trees.

### Step 1: Refactor `tb_core::ast`
*   **File:** `tb_core/src/ast.rs`
*   **Action:** 
    1. Define `pub enum SemanticType { Price, Volume, Ratio, Scalar, Boolean }`.
    2. Overhaul `pub enum Expr`. Rip out the ~100 named indicators (e.g., `Rsi`, `MacdLine`, `BollingerUpper`, all candlestick pattern variants).
    3. Replace them with the primitives defined in Section 4 of this document.
    4. Retain the hardcoded exception nodes listed in Section 4h.
    5. Implement `impl Expr { pub fn semantic_type(&self) -> SemanticType }` that returns the output type of any expression node, following the type algebra in Section 5b.
    6. Update the random tree generator (in `tb_bitwise/src/ga.rs` or equivalent) to only generate AST nodes that are type-valid according to Section 5b. When selecting operands for `Add`, `Sub`, `Mul`, `Div`, the generator must check type compatibility and reject FORBIDDEN combinations.

### Step 2: Refactor `tb_math`
*   **Action:** `tb_math` must provide calculation functions for the primitives: `sma()`, `ema()`, `wma()`, `rma()`, `std_dev()`, `true_range()`, `lin_reg_slope()`, `delay()`, `ts_max()`, `ts_min()`, `ts_sum()`. Remove all deprecated retail indicator combination functions (e.g., `macd_histogram()`, `rsi()`, `bollinger_upper()`). These are no longer needed because the AST composes them from primitives.

### Step 3: Refactor `tb_bitwise` Boolean Matrix
*   **File:** `tb_bitwise/src/precompute.rs` (or `engine.rs` / `grid.rs` equivalent)
*   **Context:** `tb_bitwise` currently pre-computes a massive boolean grid of all possible conditions (e.g. `RSI > 50`) so the Genetic Algorithm can do bitwise `AND` across boolean arrays to evaluate fitness in microseconds.
*   **Decision: Use Option B (Pre-computed Base Arrays with UI Presets).**
    The engine should pre-compute the base data arrays for each primitive input (`Close`, `Open`, `High`, `Low`, `Volume`) and common derived arrays (e.g., `Ema(Close, n)`, `StdDev(Close, n)`). 
    *   **Crucial Note on Periods:** To prevent RAM explosion, the lookback periods `n` for these time-series primitives (e.g., `[5, 10, 14, 20, 50, 100]`) will be strictly defined by **UI Configuration Presets**. The engine will only pre-compute arrays for the specific periods the user configures, and the GA will be restricted to mutating within that predefined set of periods.
    The AST evaluation then orchestrates fast vector operations over these cached base arrays. This preserves the microsecond-scale fitness evaluation.

*You may now proceed with the refactor. Start by explicitly planning the `Expr` enum modifications in `tb_core/src/ast.rs`.*

---

## 7. Open Design Dilemmas (For AI Consultation)

Before executing Step 3 (`tb_bitwise` refactor), the following architectural dilemmas must be resolved. Please review these with your AI coding partner to determine the best path forward:

### Dilemma A: The Performance Problem (Memory Allocation during AST Eval)
Because the Genetic Algorithm evaluates thousands of genomes per generation over ~100,000 candles, performance is critical. In the new system, evaluating an AST node like `Sub(Close, Sma)` requires vector math across the entire dataset. 
*   **The Problem:** If the engine allocates a new `Vec<f64>` on the heap for every temporary intermediate step in the AST tree, the RAM allocation overhead will slow the GA down by orders of magnitude (milliseconds instead of microseconds).
*   **Proposed Solution:** Implement a **Thread-Local Scratchpad Buffer Pool**. Since the GA runs in parallel via Rayon, each thread is given a few pre-allocated `Vec<f64>` buffers of size `total_bars`. The recursive AST evaluator writes intermediate math results into these reusable scratchpads instead of allocating new vectors. 
*   **Question for Claude:** Is this Scratchpad approach the most idiomatic and performant way to solve the allocation bottleneck in Rust Rayon, or is there a better zero-allocation strategy for recursive AST evaluation?

**[Opus] Recommendation:** Mostly agree with the instinct, but reconsider the performance expectations. The old system was microseconds because it did bitwise AND on pre-computed booleans — zero math. The new system does actual floating-point vector arithmetic across 100K candles per AST node. Even with perfect zero-allocation, expect **low milliseconds per genome**, not microseconds. That's still very workable (1000 genomes × 2ms = 2 seconds/generation).

**[Opus] Implementation approach:** Don't build a complex thread-local scratchpad pool upfront. Start with simple `Vec<f64>` allocation per evaluation. Rust's allocator is remarkably fast for same-size allocations — if every thread repeatedly allocates and frees `Vec<f64>` of the same 100K size, the allocator will recycle the same memory pages. Profile first, optimise second. If profiling proves allocation is the bottleneck (not the math itself), *then* add a simple `thread_local!` with a `RefCell<Vec<Vec<f64>>>` as a buffer stack — that's ~20 lines of code, not an architectural commitment. The actual bottleneck will likely be **cache misses** from random-access patterns when the AST evaluator touches multiple base arrays that don't fit in L3 cache simultaneously.

### Dilemma B: The Shape of the New Genome
Currently, a `Genome` is a list of 2 to 5 independent conditions that are implicitly `AND`ed together by the bitwise engine. When we move from bitset indexes to `Expr` trees, how should the Genome struct change?
*   **Option 1 (Single Tree):** The Genome becomes a single massive `Expr` tree. The GA generates one giant `Expr::And` node at the root that contains everything.
*   **Option 2 (Vec of Trees):** Keep the Genome as `pub conditions: Vec<Expr>`. The GA generates 2-5 independent boolean triggers (e.g. `(Close - SMA) > 0`). The engine evaluates each `Expr` into a bitset independently, and then uses the existing fast bitwise `AND` logic to merge them.
*   **Question for Claude:** Which option is better? Option 2 seems highly recommended because it keeps the AST random generator simpler (it only has to generate relational trees, not logical compounds) and allows the UI to easily display the strategy as a list of independent "Entry Rules". Do you agree?

**[Opus] Recommendation: Option 2, strongly agree.** Reasoning:
1. **Mutation sanity.** With Option 1, mutating one branch of a deeply nested `And(And(And(...)))` tree requires the GA to understand the tree's logical structure. With Option 2, the GA can independently mutate, add, or remove individual rules without touching the others.
2. **Crossover.** Option 2 gives natural crossover: take 2 rules from Parent A and 1 from Parent B. Option 1 requires sub-tree splicing across two different logical structures — significantly harder to get right.
3. **UI display.** Option 2 maps perfectly to "Entry Rule 1, Entry Rule 2, Entry Rule 3" in the UI.
4. **Backward compatibility.** The existing engine already knows how to AND multiple bitsets together. Option 2 preserves that entire code path.

**[Opus] Critical nuance:** Each individual `Expr` in the `Vec` must evaluate to `Boolean` (i.e., the root node must always be `GreaterThan`, `LessThan`, `CrossAbove`, or `CrossBelow`). The AST random generator should be structured as: *first* pick a relational operator as the root, *then* independently grow the left-hand and right-hand sub-trees using the semantic type rules. This guarantees every generated `Expr` is a valid boolean trigger without needing post-hoc validation.

### Dilemma C: Max Depth configuration
The max tree depth (which governs how complicated the mathematical formulas can get) MUST be passed dynamically from the `SessionConfig` / `Phase1Config` so the user can control it via the UI. 
*   **Question for Claude:** Should we reuse the existing "Max Rules / Complexity" slider for this, or introduce a new distinct `max_ast_depth` parameter to separate "number of rules" from "complexity of the math within a rule"?

**[Opus] Recommendation: Introduce two separate parameters.** These are fundamentally different dimensions of strategy complexity and conflating them will confuse both the user and the GA:
*   **`max_num_rules`** (existing slider, rename if needed): Controls how many independent boolean conditions are AND'd together (the length of `Vec<Expr>`). Range: 1–8. Governs *how many filters* the strategy applies.
*   **`max_ast_depth`** (new slider): Controls the maximum depth of each individual `Expr` tree. Range: 2–6. Governs *how mathematically complex* each filter can be.

**[Opus] Why separation matters:** A depth-2 rule is `Close > SMA(Close, 20)`. A depth-4 rule is `(Close - SMA(Close, 20)) / StdDev(Close, 20) > 1.5`. These are very different levels of mathematical sophistication. A user might want 5 simple rules (high `max_num_rules`, low `max_ast_depth`) for broad multi-filter strategies, or 2 deeply mathematical rules (low `max_num_rules`, high `max_ast_depth`) for focused quantitative strategies. Conflating these into one slider removes that flexibility.

---

### [Opus] Summary of Recommendations

| Dilemma | Decision |
|:---|:---|
| **A: Allocation** | Start with naive `Vec<f64>` allocation. Profile. Add `thread_local!` buffer pool only if allocation is proven bottleneck. |
| **B: Genome Shape** | **Option 2** (`Vec<Expr>` with implicit AND). Each `Expr` must have a relational operator at its root. |
| **C: Max Depth** | **Two separate UI parameters:** `max_num_rules` (how many filters) and `max_ast_depth` (how complex each filter's math is). |
