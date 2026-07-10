# Phase 1 Alpha Foundry: Architectural Audit

> **Purpose:** Self-contained reference for an implementing AI. Contains all code snippets, design doc excerpts, and fix specifications needed to resolve the performance and convergence issues without re-scanning the codebase.

---

## Executive Summary

The `tb_foundry` engine has **significant architectural drift** from the three design documents. The two user-reported symptoms (66 strats/sec throughput; strategy convergence) are caused by 7 distinct issues, listed below in priority order.

| # | Issue | Severity | Impact |
|---|-------|----------|--------|
| 1 | No MAP-Elites Archive | **Critical** | Direct cause of convergence |
| 2 | Polars `.select()` plan explosion | **Critical** | Primary speed bottleneck |
| 3 | No Hard Constraints (Death Penalties) | **High** | Wasted evals on broken strategies |
| 4 | Wrong fitness target (sum of returns vs Virtual TP/SL) | **High** | Fitness landscape is noisy, GA can't climb |
| 5 | No CPCV slicing | **Medium** | Overfitted strategies survive |
| 6 | Single-objective sort vs NSGA-II 3-objective Pareto | **Medium** | No diversity pressure in selection |
| 7 | Mutation-only evolution (no crossover, no tournament) | **Medium** | Poor genetic exploration |

---

## Issue 1: No MAP-Elites Archive (Convergence Root Cause)

### What the design says
`greenfield_generation_design.md` §Phase 3 specifies a **MAP-Elites Archive** that replaces the traditional Hall of Fame. Strategies are stored in a behavioural grid keyed by `(trade_frequency_bin, indicator_count, sketch_id)`. A new strategy only replaces an occupant if it has strictly better fitness *within that cell*. This **structurally guarantees diversity**.

**Design doc reference (`greenfield_generation_design.md` Lines 321-400):**
```rust
struct Behaviour {
    trade_frequency: f64,   // trades per 100 bars
    indicator_count: u8,
    sketch_id: u8,
}

struct Archive {
    grid: HashMap<(u8, u8, u8), EliteStrategy>,
    min_fitness: f64,
}

impl Archive {
    fn try_insert(&mut self, strategy: EliteStrategy, behaviour: Behaviour) -> bool {
        let cell = behaviour.cell();
        match self.grid.get(&cell) {
            None => { self.grid.insert(cell, strategy); true }
            Some(existing) if strategy.fitness > existing.fitness => {
                self.grid.insert(cell, strategy); true
            }
            _ => false,
        }
    }
}
```

### What the implementation does
`engine.rs` Lines 129-156 use a primitive **sort-and-cull** mechanism:

```rust
// 3. Pareto Sort (descending by fitness)
evaluated_pop.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

// 4. Record the Elite (Rank 0) for this generation
if let Some((best_sketch, best_fitness)) = evaluated_pop.first() {
    // ... send single best to Phase 2
}

// 5. Cull bottom 50% and Mutate Survivors
let survival_count = population_size / 2;
let mut survivors: Vec<Sketch> = evaluated_pop.into_iter()
    .take(survival_count).map(|(s, _)| s).collect();

let mut next_generation = Vec::with_capacity(population_size);
for i in 0..population_size {
    let mut parent = survivors[i % survival_count].clone();
    mutation_engine.mutate_sketch(&mut parent);
    parent.name = format!("gen{}_ind{}", gen_idx, i);
    next_generation.push(parent);
}
population = next_generation;
```

### Why this causes convergence
The top 50% all cluster around similar fitness. After a few generations, every survivor shares the same structural pattern. Mutation nudges parameters but cannot escape the local optimum because there is zero diversity pressure—the only selection criterion is raw fitness.

### Fix specification
1. Create `tb_foundry/src/archive.rs` with `Behaviour` and `Archive` structs per the design doc.
2. In the GA loop, after evaluation, call `archive.try_insert()` for each evaluated strategy.
3. Replace the "cull bottom 50%" block with: sample parents from archive elites + tournament selection from the current population.
4. Re-inject archive elites into each generation (the design specifies `population_size / 10` elites per generation).

---

## Issue 2: Polars `.select()` Plan Explosion (Speed Bottleneck)

### Current code (`engine.rs` Lines 63-81)
```rust
let mut metric_exprs = Vec::with_capacity(population_size * 2);
let mut valid_sketches = Vec::new();

for sketch in population.into_iter() {
    if let Ok(polars_expr) = tb_core::ast_compiler::compile_ast_to_polars(&sketch.entry_long) {
        let (fit_expr, risk_expr) = crate::metrics::build_metric_expressions(polars_expr, &sketch.name);
        metric_exprs.push(fit_expr);
        metric_exprs.push(risk_expr);
        valid_sketches.push(sketch);
    }
}

// ALL expressions evaluated in a single .select() call
let results_df = prepared_data.clone().lazy()
    .select(metric_exprs)   // <-- population_size * 2 columns
    .collect()?;
```

### Why it's slow
With `population_size = 100`, this passes **200 Polars expressions** into a single `.select()`. Polars' query optimizer must build a DAG, detect common subexpressions, and plan parallelism for all 200 simultaneously. The optimizer complexity is super-linear in the number of expressions. At `population_size = 1000` (needed for real throughput), this would pass 2000 columns and the optimizer would choke entirely.

### Fix specification
**Option A (Quick fix):** Chunk the expressions into batches of ~50 and run multiple smaller `.select().collect()` calls. Each batch runs in parallel internally via Rayon.

```rust
let batch_size = 50; // 25 strategies * 2 metrics each
for chunk in metric_exprs.chunks(batch_size) {
    let batch_df = prepared_data.clone().lazy()
        .select(chunk.to_vec())
        .collect()?;
    // extract results from batch_df...
}
```

**Option B (Ideal, per design doc):** Switch to the **Signal Matrix × Target Vector** architecture from `alpha_target_engineering.md`. Instead of building N separate Polars expression trees, compile each strategy's boolean signal into a single column, stack them into a matrix, and multiply against the pre-computed target vector. This is a single matrix operation regardless of population size.

---

## Issue 3: No Hard Constraints (Death Penalties)

### What the design says (`alpha_target_engineering.md` Lines 138-158)
Four mandatory constraints must be checked **before** fitness evaluation:

| Constraint | Rule | Kill if... |
|------------|------|-----------|
| Min Event Count | Must fire > 50 trades | Too few signals |
| Max Exposure | Cannot be in market > 40% of bars | Signal is always on (Beta trap) |
| Directional Symmetry | Long/Short can't be > 90%/10% skewed | Lazy "always long" |
| Zero Variance | Signal is uniformly `[1,1,1...]` or `[0,0,0...]` | Broken AST logic |

### What the implementation does
Nothing. Every compiled strategy goes through the full Polars metric evaluation regardless of whether it's a "Broken Clock" (e.g., `Close > 0` which is always true).

### Fix specification
After compiling each AST to a Polars expression, evaluate the boolean signal column alone (cheap) and check:
```rust
let signal_series = prepared_data.clone().lazy()
    .select([signal_expr.clone().alias("signal")])
    .collect()?;
let signal_col = signal_series.column("signal")?.bool()?;
let true_count = signal_col.sum().unwrap_or(0) as f64;
let total = signal_col.len() as f64;
let exposure = true_count / total;

if true_count < 50.0 || exposure > 0.40 || exposure < 0.001 {
    // Death penalty: skip expensive metric evaluation
    fitness = -9999.0;
    continue;
}
```
This avoids the expensive metric computation for strategies that are structurally broken.

---

## Issue 4: Wrong Fitness Target

### What the design says (`alpha_target_engineering.md` §5)
Phase 1 should use a **Virtual TP/SL Binary Target**:
> *"For every bar `i`, look forward. Does the price hit `+2 ATR` before it hits `-1 ATR`? If Yes, `Target[i] = 1`. If No, `Target[i] = -1`."*

The GA optimizes for **hit rate**: *"When this signal fires, what is the probability it hits a 2R profit before a 1R loss?"*

### What the implementation does (`metrics.rs`)
```rust
let strat_ret = signal_expr.cast(DataType::Float64) * col("forward_return");

// Fitness Proxy: Sum of returns
let total_return = (strat_ret.clone().sum() * lit(100.0)).alias(&format!("ret_{}", strat_id));

// Risk Proxy: Volatility (Standard Deviation)
let risk = (strat_ret.std(1) * lit(100.0)).alias(&format!("max_dd_{}", strat_id));
```

And `forward_return` is just `(close[t+1] - close[t]) / close[t]`—a single-bar raw percentage return.

### Why this is wrong
1. **Single-bar horizon** means the GA optimizes for next-bar prediction. This creates massive noise sensitivity and rewards strategies that happen to align with random 1-bar moves.
2. **Sum of returns** is dominated by a few large outlier bars. A strategy that fires on 3 bars where the market moved +5% each will beat a strategy that fires on 200 bars with consistent +0.1%. The first is curve-fit noise; the second is actual alpha.
3. **No ATR normalization** means strategies that fire during volatile periods appear to have higher fitness purely from beta exposure, not alpha.

### Fix specification
Pre-compute the Virtual TP/SL target array once before the GA loop:
```rust
// In prepare_data_for_evaluation():
// For each bar, look forward N bars. Did price hit +2*ATR before -1*ATR?
// target = +1 if TP hit first, -1 if SL hit first, 0 if neither
```
Then fitness becomes: `signal.cast(f64) * target_col` → `.sum()`. This is a single dot-product per strategy.

---

## Issue 5: No CPCV Slicing

### What the design says (`core_pipeline_architecture.md` §3, `alpha_target_engineering.md` §7)
> *"Instead of evaluating the matrix over the entire dataset, we pre-slice the Target Vector into N chronological chunks (e.g., 6 chunks). The matrix multiplication evaluates the strategy against all combinations of those chunks."*
> *"Phase 1 fitness is not just 'Total Return'. It is the variance of performance across the CPCV paths."*

### What the implementation does
Evaluates across the **entire dataset** with no slicing. A strategy that makes 100% in 2020 and -50% in 2021 gets the same fitness as one that makes 25% consistently across both.

### Fix specification
1. Pre-slice the materialized DataFrame into N chunks (e.g., 6).
2. Evaluate each strategy across multiple train/test combinations.
3. Use **variance across folds** as the second Pareto objective (low variance = robust).

---

## Issue 6: Single-Objective Sort vs 3-Objective Pareto

### What the design says (`alpha_target_engineering.md` Lines 95-99)
Three Pareto objectives via NSGA-II:
1. **Maximize Raw Edge** (continuous cumulative return of Virtual TP/SL)
2. **Minimize CPCV Variance** (regime robustness)
3. **Dynamic User Goal** (complexity, MAE, or win/loss ratio from UI config)

### What the implementation does
```rust
evaluated_pop.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
```
Single scalar sort on raw fitness. No Pareto ranking, no multi-objective selection.

### Fix specification
Implement NSGA-II non-dominated sorting with crowding distance. Each strategy gets 3 scores; selection operates on Pareto rank + crowding distance rather than raw fitness.

---

## Issue 7: Mutation-Only Evolution (No Crossover, No Tournament)

### What the design says (`greenfield_generation_design.md` Lines 483-508)
```rust
// Fill rest with tournament selection + crossover + mutation
while next_gen.len() < config.population_size {
    let parent1 = tournament_selection(&fitness_pop, config.tournament_size, &mut rng);
    let parent2 = tournament_selection(&fitness_pop, config.tournament_size, &mut rng);
    let (mut child1, mut child2) = crossover(&parent1, &parent2, &mut rng);
    mutate(&mut child1, config.mutation_rate, &mut rng);
    mutate(&mut child2, config.mutation_rate, &mut rng);
    next_gen.push((child1, f64::NEG_INFINITY));
}
```

### What the implementation does (`engine.rs` Lines 148-156)
```rust
let mut next_generation = Vec::with_capacity(population_size);
for i in 0..population_size {
    let mut parent = survivors[i % survival_count].clone();
    mutation_engine.mutate_sketch(&mut parent);
    parent.name = format!("gen{}_ind{}", gen_idx, i);
    next_generation.push(parent);
}
```

No tournament selection. No crossover. Just clone-and-mutate from the survivor pool in round-robin order. This means:
- **No recombination** of successful traits from different strategies
- **No selection pressure** beyond the initial sort-and-cull
- Parents are chosen deterministically (round-robin), not fitness-proportionally

### Fix specification
1. Implement tournament selection (pick K random candidates, return the fittest).
2. Implement AST crossover (swap subtrees between two parent sketches).
3. Apply crossover first, then mutation, per the standard GA pipeline.

---

## Prioritized Implementation Order

| Priority | Action | Files to Create/Modify |
|----------|--------|----------------------|
| **P0** | Implement MAP-Elites Archive | Create `tb_foundry/src/archive.rs`, modify `engine.rs` |
| **P0** | Batch the `.select()` calls | Modify `engine.rs` eval loop |
| **P1** | Add Hard Constraints (Death Penalties) | Modify `engine.rs` eval loop |
| **P1** | Replace fitness with Virtual TP/SL target | Rewrite `metrics.rs`, modify `prepare_data_for_evaluation` |
| **P2** | Add tournament selection + crossover | Create `tb_foundry/src/operators.rs`, modify `engine.rs` |
| **P2** | Add CPCV slicing | Modify `metrics.rs` and `engine.rs` |
| **P3** | Implement NSGA-II 3-objective Pareto sort | Create `tb_foundry/src/pareto.rs`, modify `engine.rs` |

---

## Full File Inventory (Current State)

### `tb_foundry/src/` (6 files, ~410 lines total)
| File | Lines | Purpose |
|------|-------|---------|
| `engine.rs` | 165 | GA loop, evaluation, selection |
| `metrics.rs` | 33 | Fitness/risk Polars expressions |
| `ga.rs` | 114 | MutationEngine (tree-aware AST mutation) |
| `sketches.rs` | 85 | Seed population generator (3 templates) |
| `error.rs` | 11 | FoundryError enum |
| `lib.rs` | 9 | Module exports |

### `tb_core/src/` (6 files, ~330 lines total)
| File | Lines | Purpose |
|------|-------|---------|
| `ast.rs` | 115 | Expr enum + Sketch struct + Display impls |
| `ast_compiler.rs` | 94 | AST → Polars expression compiler |
| `indicators.rs` | 56 | SMA, EMA, RSI, MACD implementations |
| `config.rs` | 98 | SessionConfig, Phase1Config, Phase2Config |
| `error.rs` | 11 | CoreError enum |
| `lib.rs` | 9 | Module exports |
