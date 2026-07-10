# Genetic Algorithm (GA) Engine Design

> **Context:** Phase 1 uses a Genetic Algorithm to discover alpha. While `alpha_target_engineering.md` defines how strategies are *scored* (the matrix math and Pareto sort), this document defines how they are *generated and mutated*.

Because we use a recursive JSON AST (`Expr`) rather than a flat array of integers, the GA's mutation and crossover operators must be tree-aware.

---

## 1. The Sketch Library (Seed Generation)

To initialize Generation 0 (the seed population of 100,000 strategies), `tb_foundry` does not generate completely random, unconstrained trees. It uses a **Sketch Library**—a set of procedural Rust templates that generate valid AST branches.

### The Base Catalogue
1. **Trend Cross:** `CrossAbove(Fast_MA, Slow_MA)`
2. **Mean Reversion:** `LessThan(Oscillator, Oversold_Threshold)`
3. **Volatility Breakout:** `GreaterThan(Close, Upper_Bollinger_Band)`
4. **Confluence (Complex):** `And(Trend_Cross, Mean_Reversion)`

When the user clicks "Start", the GA populates Generation 0 by randomly selecting Sketches from this library and assigning random integer/float parameters (e.g., picking `Trend Cross`, assigning Fast=10, Slow=50, picking `EMA` as the MA type).

---

## 2. Tree-Aware Mutation Operators

During evolution, the GA selects elite strategies and mutates them to create the next generation. Because the genome is an `Expr` tree, we use **Genetic Programming (GP)** mutation operators.

### A. Parameter Mutation (The Micro Shift)
* **Action:** Randomly nudges a primitive value.
* **Example:** `period: 14` mutates to `period: 17`. 
* **Frequency:** High (~70% of mutations).

### B. Node Swapping (The Lateral Shift)
* **Action:** Swaps a node for a mathematically compatible sibling without altering the children.
* **Example:** A `Sma { source: Close }` mutates into an `Ema { source: Close }`.
* **Frequency:** Medium (~20% of mutations).

### C. Branch Pruning & Growing (The Macro Shift)
* **Action:** Alters the complexity of the AST. 
* **Grow Example:** Replaces `Close` with `Sma { source: Close, period: 10 }`.
* **Prune Example:** Replaces an `And { lhs, rhs }` entirely with just `lhs` (simplifying the strategy).
* **Frequency:** Low (~10% of mutations). Used to explore radical new logic, bounded by the user's `ComplexityCap`.

---

## 3. Crossover Operators

Crossover involves taking two parent `Expr` trees and combining them.
* **Subtree Crossover:** The engine picks a random node in Parent A and a random node of the same return type (e.g., both return `bool` or both return `f64`) in Parent B. It swaps the subtrees.
* **Safety Guarantee:** Because `Expr` is strictly typed, swapping a `bool` node (like `CrossAbove`) for another `bool` node (like `LessThan`) always results in a structurally valid strategy that will compile and evaluate cleanly in Polars.

---

## 4. Population Management & Selection

* **Population Size:** 100,000 strategies per generation.
* **Elitism:** The top 5% of the Rank 0 Pareto frontier is passed entirely unmodified to the next generation to guarantee the system never loses its best discoveries.
* **Selection Pressure:** **Tournament Selection**. To pick parents for mutation/crossover, the engine randomly selects 5 strategies from the population and chooses the one with the highest Pareto rank. This maintains diversity while applying steady upward pressure.
* **Epochs:** The GA runs for a maximum of 500 generations, or until the Pareto frontier stops expanding (Early Termination).
