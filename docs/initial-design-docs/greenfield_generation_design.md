# Greenfield Strategy Generation: Design Document

> **Premise:** We are building the strategy generation engine from scratch. No legacy code constraints. The design is informed by lessons learned from the current system, but every decision is made fresh for simplicity and correctness.

---

## Design Principles

1. **If it compiles, it's valid.** The type system and construction API make it impossible to build a nonsensical strategy. No post-hoc validators.
2. **One way to do things.** No parallel `SemanticMapper` vs `GuidedMapper`. One builder that handles everything.
3. **Composition over configuration.** Sketches compose like legos. No god-object registries with 40+ entries.
4. **The genome is the source of randomness, not structure.** Genomes choose *which* sketch and *what* parameters, never *how* nodes connect.

---

## Architecture Overview

```
                     ┌──────────────────────┐
                     │     SketchLibrary     │
                     │                       │
                     │  ┌─────────────────┐  │
                     │  │ OscThreshold    │  │
                     │  │ TrendCross      │  │
                     │  │ VolBreakout     │  │
                     │  │ Confluence(A,B) │  │
                     │  │ ... (promoted)  │  │
                     │  └─────────────────┘  │
                     └──────────┬───────────┘
                                │ picks sketch + fills params
                                │
Genome ──► GeneReader ──► SketchResolver ──► Strategy
(Vec<u32>)                                   (always valid)
                                                │
                                                ▼
                                           Backtester
                                                │
                                                ▼
                                          ┌───────────┐
                                          │ MAP-Elites │
                                          │  Archive   │
                                          └───────────┘
```

---

## Phase 1: Type-Safe Construction

### Core Types

```rust
/// The scale of a numeric value. This is the key type that prevents nonsense.
/// Two values can only be compared if their scales are compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Scale {
    /// Absolute price level (Close, SMA, EMA, BollingerBand)
    Price,
    /// Bounded 0–100 (RSI, Stochastic, Williams%R, MFI)
    Bounded,
    /// Unbounded centered on zero (MACD, CCI, Momentum, ROC)
    Centered,
    /// Positive unbounded (ATR, StdDev, ADX)
    Volatility,
    /// Volume-scale (OBV, Volume, Force)
    Volume,
}

impl Scale {
    /// Can this scale be meaningfully compared to a scalar constant?
    fn allows_threshold(&self) -> bool {
        matches!(self, Scale::Bounded | Scale::Centered | Scale::Volatility)
    }

    /// Can two scales be compared to each other?
    fn is_compatible(&self, other: &Scale) -> bool {
        self == other
    }

    /// Valid constant range for threshold comparisons
    fn threshold_range(&self) -> (f64, f64) {
        match self {
            Scale::Bounded     => (5.0, 95.0),
            Scale::Centered    => (-200.0, 200.0),
            Scale::Volatility  => (0.0001, 10.0),
            Scale::Price       => unreachable!("price doesn't use thresholds"),
            Scale::Volume      => unreachable!("volume doesn't use thresholds"),
        }
    }
}
```

### Indicator Registry (Declarative, Minimal)

Instead of 43 separate files each implementing a trait, indicators are declared as data:

```rust
struct IndicatorDef {
    name: &'static str,          // "RSI"
    scale: Scale,                // Scale::Bounded
    category: Category,          // Category::Momentum
    params: &'static [ParamDef],
    spec: fn(&[f64]) -> IndicatorSpec,  // builds the DSL spec with given params
}

struct ParamDef {
    name: &'static str,
    default: f64,
    min: f64,
    max: f64,
    step: f64,  // quantisation step (1.0 for integers)
}

/// All indicators defined in one place
const INDICATORS: &[IndicatorDef] = &[
    IndicatorDef {
        name: "RSI",
        scale: Scale::Bounded,
        category: Category::Momentum,
        params: &[ParamDef { name: "period", default: 14.0, min: 2.0, max: 200.0, step: 1.0 }],
        spec: rsi_spec,
    },
    IndicatorDef {
        name: "SMA",
        scale: Scale::Price,
        category: Category::Trend,
        params: &[ParamDef { name: "period", default: 20.0, min: 2.0, max: 500.0, step: 1.0 }],
        spec: sma_spec,
    },
    // ... every indicator is just one entry here
];
```

> **Why this is simpler:** The current system has 43 indicator files, a `FunctionRegistry`, a `MetadataRegistry`, a `StrategyFunction` enum, an `Indicator` trait, a `Primitive` trait, and `get_indicators()` in `mod.rs` that duplicates `register_indicators()` in `registry.rs`. All of that collapses into one `const` array.

### Typed Expressions

Expressions carry their scale at the type level. You literally cannot construct an invalid comparison:

```rust
/// A numeric series with a known scale
struct Series {
    node: Expr,     // the computation (polars expression or AST)
    scale: Scale,   // what kind of values this produces
}

/// A boolean condition (the output of a comparison)
struct Condition {
    node: Expr,
}

/// Comparison constructors — these are the ONLY way to create conditions
impl Condition {
    /// Compare two series of the same scale
    fn compare(left: &Series, op: CmpOp, right: &Series) -> Result<Condition> {
        if !left.scale.is_compatible(&right.scale) {
            bail!("Cannot compare {:?} to {:?}", left.scale, right.scale);
        }
        Ok(Condition { node: Expr::Compare(left.node, op, right.node) })
    }

    /// Compare a series against a scalar threshold (only for scales that allow it)
    fn threshold(series: &Series, op: CmpOp, value: f64) -> Result<Condition> {
        if !series.scale.allows_threshold() {
            bail!("{:?} cannot be compared to a constant", series.scale);
        }
        let (min, max) = series.scale.threshold_range();
        if value < min || value > max {
            bail!("{} is outside valid range [{}, {}] for {:?}", value, min, max, series.scale);
        }
        Ok(Condition { node: Expr::Threshold(series.node, op, value) })
    }

    /// Combine two conditions with AND
    fn and(a: Condition, b: Condition) -> Condition {
        Condition { node: Expr::And(a.node, b.node) }
    }

    /// Combine two conditions with OR
    fn or(a: Condition, b: Condition) -> Condition {
        Condition { node: Expr::Or(a.node, b.node) }
    }
}
```

> **Key insight:** There is no `ConstraintValidator`, no `ScaleCompatibilityConstraint`, no `validate_comparison()`. If you can call the function, it's valid. The compiler enforces this.

---

## Phase 2: Strategy Sketches

### What is a Sketch?

A sketch is a **structural pattern** with **typed holes** that the genome fills in.

```rust
/// A hole in a sketch that the genome fills
enum Slot {
    /// Pick an indicator from a filtered set
    Indicator {
        scale: Option<Scale>,      // if Some, only indicators of this scale
        category: Option<Category>, // if Some, only this category
    },
    /// Pick a parameter value within bounds
    Param { min: f64, max: f64, step: f64 },
    /// Pick a comparison operator
    CmpOp { allowed: &'static [CmpOp] },
    /// Pick a direction (Long or Short)
    Direction,
}

/// A sketch defines the shape of a strategy
struct Sketch {
    name: &'static str,
    build: fn(&mut GeneReader, &IndicatorTable) -> Result<Strategy>,
}
```

### Built-in Sketches

```rust
/// "RSI(14) < 30 → Long"  (oscillator vs threshold)
fn oscillator_threshold(genes: &mut GeneReader, indicators: &IndicatorTable) -> Result<Strategy> {
    // Pick an oscillator (RSI, Stochastic, Williams%R, MFI, DeMarker...)
    let ind_def = genes.pick_indicator(indicators, Some(Scale::Bounded), None)?;
    let params = genes.fill_params(ind_def)?;
    let series = Series::from_indicator(ind_def, &params);

    // Pick comparison and threshold
    let op = genes.pick(&[CmpOp::Gt, CmpOp::Lt]);
    let threshold = genes.float_in_range(series.scale.threshold_range());

    let condition = Condition::threshold(&series, op, threshold)?;  // infallible by construction
    let direction = genes.pick_direction();

    Ok(Strategy::new(condition, direction))
}

/// "SMA(20) cross_above SMA(50) → Long"  (trend cross)
fn trend_cross(genes: &mut GeneReader, indicators: &IndicatorTable) -> Result<Strategy> {
    // Pick two price-scale indicators (SMA, EMA, DEMA, TEMA, BB, Envelope...)
    let fast_def = genes.pick_indicator(indicators, Some(Scale::Price), Some(Category::Trend))?;
    let slow_def = genes.pick_indicator(indicators, Some(Scale::Price), Some(Category::Trend))?;
    let fast_params = genes.fill_params(fast_def)?;
    let slow_params = genes.fill_params(slow_def)?;
    let fast = Series::from_indicator(fast_def, &fast_params);
    let slow = Series::from_indicator(slow_def, &slow_params);

    let op = genes.pick(&[CmpOp::CrossAbove, CmpOp::CrossBelow, CmpOp::Gt, CmpOp::Lt]);
    let condition = Condition::compare(&fast, op, &slow)?;  // infallible: both are Scale::Price
    let direction = genes.pick_direction();

    Ok(Strategy::new(condition, direction))
}

/// "ATR(14) > threshold AND RSI(14) < 30 → Long"  (confluence of two sketches)
fn confluence(genes: &mut GeneReader, indicators: &IndicatorTable) -> Result<Strategy> {
    // Pick two different base sketches and AND them together
    let sketch_a = genes.pick_sketch(&BASE_SKETCHES);
    let sketch_b = genes.pick_sketch(&BASE_SKETCHES);

    let strategy_a = (sketch_a.build)(genes, indicators)?;
    let strategy_b = (sketch_b.build)(genes, indicators)?;

    let condition = Condition::and(strategy_a.condition, strategy_b.condition);
    let direction = strategy_a.direction; // use direction from first sketch

    Ok(Strategy::new(condition, direction))
}
```

### The Sketch Library

```rust
/// Base sketches (non-composite)
const BASE_SKETCHES: &[Sketch] = &[
    Sketch { name: "oscillator_threshold", build: oscillator_threshold },
    Sketch { name: "trend_cross",          build: trend_cross },
    Sketch { name: "volatility_breakout",  build: volatility_breakout },
    Sketch { name: "volume_confirmation",  build: volume_confirmation },
    Sketch { name: "momentum_divergence",  build: momentum_divergence },
    Sketch { name: "mean_reversion_band",  build: mean_reversion_band },
];

/// All sketches (base + composite)
const ALL_SKETCHES: &[Sketch] = &[
    // ...all base sketches...
    Sketch { name: "confluence",     build: confluence },
    Sketch { name: "either_trigger", build: either_trigger },  // OR variant
];
```

> **Why this is simpler:** The current system has `TemplateId` enum, `StrategyTemplate` trait, 7 archetype structs each with 7 method implementations, a `TemplateRegistry`, a `CompositeTemplate`, `GuidedMapper` (800+ lines), and `ConstraintValidator` (400+ lines). All of that becomes a handful of plain functions in a `sketches.rs` file. Each sketch is 10–20 lines of code that reads like pseudocode.

### Sketch Promotion (Self-Improvement)

When the evolution engine discovers a high-performing strategy, its structure can be extracted as a new sketch:

```rust
/// Extract the structural pattern from a successful strategy
fn promote_to_sketch(strategy: &Strategy) -> DynamicSketch {
    // Walk the AST and replace concrete indicators/params with slots
    // e.g., "RSI(14) < 30" becomes "Indicator[Bounded](Param[2..200]) CmpOp Param[5..95]"
    let pattern = extract_pattern(&strategy.condition);

    DynamicSketch {
        name: format!("promoted_{}", strategy.fingerprint()),
        pattern,
        source_fitness: strategy.fitness,
    }
}

struct SketchLibrary {
    builtin: &'static [Sketch],          // the const array above
    promoted: Vec<DynamicSketch>,         // discovered during evolution
    max_promoted: usize,                 // cap to prevent unbounded growth
}
```

---

## Phase 3: MAP-Elites Archive

### Replacing the Hall of Fame

The current `HallOfFame` keeps the N best strategies by fitness (with Pareto ranking). This means convergence — all top strategies tend to look alike.

MAP-Elites replaces this with a **grid** where each cell holds the best strategy that exhibits a particular **behaviour**.

### Behavioural Dimensions

```rust
/// Features that describe a strategy's behaviour (not its quality)
struct Behaviour {
    /// How often it trades (trades per 100 bars)
    trade_frequency: f64,
    /// How many distinct indicators it uses
    indicator_count: u8,
    /// Which sketch archetype it came from
    sketch_id: u8,
}

/// Discretise behaviour into a grid cell
impl Behaviour {
    fn cell(&self) -> (u8, u8, u8) {
        let freq_bin = match self.trade_frequency {
            f if f < 1.0  => 0,  // rare trader
            f if f < 5.0  => 1,  // moderate
            f if f < 15.0 => 2,  // active
            _             => 3,  // hyperactive
        };
        let ind_bin = self.indicator_count.min(4);  // 0,1,2,3,4+
        let sketch_bin = self.sketch_id;

        (freq_bin, ind_bin, sketch_bin)
    }
}
```

### The Archive

```rust
struct Archive {
    /// Grid: cell → best strategy in that cell
    grid: HashMap<(u8, u8, u8), EliteStrategy>,
    /// Quality threshold: only accept strategies with fitness above this
    min_fitness: f64,
}

impl Archive {
    /// Try to add a strategy. Returns true if it was added.
    fn try_insert(&mut self, strategy: EliteStrategy, behaviour: Behaviour) -> bool {
        if strategy.fitness < self.min_fitness {
            return false;
        }

        let cell = behaviour.cell();

        match self.grid.get(&cell) {
            None => {
                // Empty cell — this is a novel behaviour, always accept
                self.grid.insert(cell, strategy);
                true
            }
            Some(existing) if strategy.fitness > existing.fitness => {
                // Better than current occupant — replace
                self.grid.insert(cell, strategy);
                true
            }
            _ => false,  // cell occupied by better strategy
        }
    }

    /// Get all elite strategies (one per cell)
    fn elites(&self) -> Vec<&EliteStrategy> {
        self.grid.values().collect()
    }
}
```

> **Why this is simpler:** The current `HallOfFame` is 300+ lines with `seen_signatures`, Pareto ranking integration, similarity thresholds, crowding distance, deduplication, and `StrategySource` tracking. The MAP-Elites archive is ~50 lines and *structurally guarantees diversity* without needing similarity calculations.

---

## Module Structure

```
src/
├── generation/
│   ├── mod.rs              # public API: run_evolution()
│   ├── types.rs            # Scale, Category, CmpOp, Expr, Series, Condition, Strategy
│   ├── indicators.rs       # const INDICATORS table + IndicatorDef, ParamDef
│   ├── sketches.rs         # Sketch fns: oscillator_threshold, trend_cross, confluence...
│   ├── sketch_library.rs   # SketchLibrary with builtin + promoted sketches
│   ├── gene_reader.rs      # GeneReader: deterministic consumption of genome values
│   ├── evolution.rs        # GA loop: selection, crossover, mutation
│   ├── archive.rs          # MAP-Elites archive (replaces HallOfFame)
│   └── operators.rs        # tournament_selection, crossover, mutate
```

### What Each File Does

| File | Lines (est.) | Responsibility |
|---|---|---|
| `types.rs` | ~100 | Core types (`Scale`, `Condition`, `Series`). The type system that makes invalid strategies unrepresentable. |
| `indicators.rs` | ~150 | Declarative indicator table. One `const` array. Each indicator is one struct literal. |
| `sketches.rs` | ~200 | 6–8 sketch functions, each 15–25 lines. Plain Rust functions, no traits. |
| `sketch_library.rs` | ~80 | Manages builtin + promoted sketches. Sketch promotion logic. |
| `gene_reader.rs` | ~60 | Consumes genes from genome. `pick()`, `float_in_range()`, `fill_params()`. |
| `evolution.rs` | ~250 | The GA loop. Init population → evaluate → select → reproduce → repeat. |
| `archive.rs` | ~80 | MAP-Elites grid. `try_insert()`, `elites()`. |
| `operators.rs` | ~60 | `tournament_selection()`, `crossover()`, `mutate()`. Unchanged from current. |
| **Total** | **~980** | |

> **Comparison:** The current `engines/generation/` directory contains ~3,500 lines across 16 files + a `templates/` subdirectory with 5 more files (~2,000 lines). The greenfield design achieves the same (and more, with MAP-Elites) in under 1,000 lines.

---

## The Evolution Loop (Simplified)

```rust
fn run_evolution(
    config: &EvolutionConfig,
    indicators: &IndicatorTable,
    sketches: &SketchLibrary,
    data: &DataFrame,
    backtester: &Backtester,
) -> Archive {
    let mut rng = StdRng::seed_from_u64(config.seed);
    let mut archive = Archive::new(config.min_fitness);

    // 1. Initial population
    let mut population: Vec<(Genome, f64)> = (0..config.population_size)
        .map(|_| {
            let genome = random_genome(config.genome_length, &mut rng);
            (genome, f64::NEG_INFINITY)
        })
        .collect();

    // 2. Evolution loop
    for gen in 0..config.generations {
        // Evaluate
        let evaluated: Vec<_> = population
            .par_iter()
            .filter_map(|(genome, _)| {
                let mut reader = GeneReader::new(genome);
                let sketch = reader.pick_sketch(&sketches.all());
                let strategy = (sketch.build)(&mut reader, indicators).ok()?;
                let result = backtester.run(&strategy, data).ok()?;
                Some((genome.clone(), strategy, result))
            })
            .collect();

        // Archive + fitness assignment
        for (genome, strategy, result) in &evaluated {
            let behaviour = Behaviour::from_result(result);
            archive.try_insert(
                EliteStrategy::new(strategy, genome, result),
                behaviour,
            );
        }

        // Selection + reproduction
        let fitness_pop: Vec<_> = evaluated
            .iter()
            .map(|(g, _, r)| (g.clone(), r.fitness))
            .collect();

        let mut next_gen = Vec::with_capacity(config.population_size);

        // Elitism: re-inject archive elites
        for elite in archive.elites().iter().take(config.population_size / 10) {
            next_gen.push((elite.genome.clone(), elite.fitness));
        }

        // Fill rest with tournament selection + crossover + mutation
        while next_gen.len() < config.population_size {
            let parent1 = tournament_selection(&fitness_pop, config.tournament_size, &mut rng);
            let parent2 = tournament_selection(&fitness_pop, config.tournament_size, &mut rng);
            let (mut child1, mut child2) = crossover(&parent1, &parent2, &mut rng);
            mutate(&mut child1, config.mutation_rate, &mut rng);
            mutate(&mut child2, config.mutation_rate, &mut rng);
            next_gen.push((child1, f64::NEG_INFINITY));
            if next_gen.len() < config.population_size {
                next_gen.push((child2, f64::NEG_INFINITY));
            }
        }

        population = next_gen;

        // Sketch promotion: extract patterns from best new discoveries
        if gen % 10 == 0 {
            for elite in archive.top_n(3) {
                sketches.maybe_promote(&elite.strategy);
            }
        }
    }

    archive
}
```

---

## Phase 4: Dynamic User-Defined Logic (YAML/LLM)

A massive advantage of this architecture is that **indicators are completely declarative**, leveraging the `IndicatorSpec` and `CalculationStep` DSL.

Because calculation steps (`Sma`, `Add`, `ConditionalComparison`) are just enums rather than raw Rust code, users and LLMs can define new indicators in YAML without compiling anything:

```yaml
# ~/.star/indicators/pmo.yaml
name: "PMO"
scale: Centered
category: Momentum
params:
  - name: fast_period
    type: integer
    default: 10
calculations:
  - Ema:
      source: Close
      period: param(fast_period)
      output: "pmo_line"
output: "pmo_line"
```

### Extending this to Sketches
The same applies to sketches. Since sketches are just structural slots (e.g., "pick a `Bounded` indicator"), an LLM can define a `DynamicSketch` in YAML:

```yaml
# ~/.star/sketches/oversold_volume.yaml
name: "Oversold_Volume_Expansion"
slots:
  osc_period: { type: integer, min: 5, max: 50 }
  osc_threshold: { type: float, min: 5.0, max: 40.0 }
condition:
  and:
    - lt:
        - indicator: { scale: Bounded, params: [ $osc_period ] }
        - $osc_threshold
```

**The Two-Tier LLM Workflow:**
1. LLM invents 50 novel indicators (using the `CalculationStep` math DSL) and saves them as YAML.
2. LLM invents 20 structural sketches and saves them as YAML.
3. The Rust genetic engine loads all of them dynamically and cross-pollinates them, doing the computational heavy lifting to find edge-case combinations.

---

## What We Gain vs Current System

| Concern | Current System | Greenfield |
|---|---|---|
| **Nonsense strategies** | Post-hoc `ConstraintValidator` with 5 constraint classes | Impossible by construction (`Condition::compare` enforces scale) |
| **Novelty** | 7 hardcoded template archetypes | Sketch composition + sketch promotion from evolved strategies |
| **Bandaid fixes** | `ScaleCompatibilityConstraint`, `IndicatorCategoryConstraint`, `LogicalCoherenceConstraint`, `ParameterValidityConstraint`, indicator budget tracker, scale-aware constant generation | None needed. `Scale` type + sketch structure = always valid. |
| **Indicator registration** | `FunctionRegistry` + `MetadataRegistry` + `mod.rs::get_indicators()` + 43 files | One `const INDICATORS` table |
| **Diversity** | `HallOfFame` with similarity hashing + Pareto ranking | MAP-Elites archive (structural diversity guarantee) |
| **Code volume** | ~5,500 lines across 21 files | ~1,000 lines across 8 files |
| **Adding a new indicator** | Create file, impl `Indicator` trait (8 methods), register in `registry.rs`, register in `mod.rs::get_indicators()`, add metadata to `MetadataRegistry` | Add one entry to `const INDICATORS` |
| **Adding a new strategy pattern** | Create archetype struct, impl `StrategyTemplate` trait (7 methods), register in `create_all_templates()`, update `TemplateId` enum, add `from_u8` match arm | Write one `fn` in `sketches.rs` |

---

## Implementation Order

### Step 1: `types.rs` + `indicators.rs`
Define `Scale`, `Series`, `Condition`, `Strategy`, and the indicator table. Write unit tests proving that invalid comparisons are compile-time or runtime errors.

### Step 2: `gene_reader.rs` + `sketches.rs`
Implement `GeneReader` and 3 initial sketches: `oscillator_threshold`, `trend_cross`, `confluence`. Verify every genome produces a valid strategy.

### Step 3: `evolution.rs` + `operators.rs`
Port the GA loop (selection, crossover, mutation). This is mostly unchanged from the current `operators.rs`.

### Step 4: `archive.rs`
Implement MAP-Elites. Define behavioural dimensions. Wire into evolution loop.

### Step 5: `sketch_library.rs`
Add remaining sketches. Implement sketch promotion. Add the remaining indicator entries to the table.

### Step 6: Integration
Wire the new generation engine into the backtester and UI. Deprecate old `engines/generation/`.
