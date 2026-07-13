# Implementation Status & Master Roadmap

This document serves as the ground-truth reference for the current state of the Greenfield AI workspace. It outlines what has been fully built, what is partially implemented, and what is explicitly stubbed or deferred for later.

**Current Primary Focus: Phase 1 (Alpha Generation / Bitwise Evolution)**

---

## Workspace Crates

### 1. `tb_core` (Core Definitions & AST)
- **Status:** **IMPLEMENTED**
- **Contents:**
  - `ast.rs`: The central `Sketch` JSON AST representing strategy logic. `EliteStrategy` struct for passing results.
  - `config.rs`: `SessionConfig`, `Phase1Config`, and `Phase2Config`. Includes `starting_equity` (moved to Phase 1).
  - `ast_compiler.rs`: Translates the JSON AST into executable Polars Expressions.

### 2. `tb_math` (Mathematical Indicators)
- **Status:** **IMPLEMENTED (Pending Audit)**
- **Contents:**
  - Contains ~60 custom technical indicators. 
  - **Recent Fixes:** Arithmetic underflows (`i - period`) have been patched to `i + 1 - period`.
  - **Remaining Work:** A strict mathematical parity audit against `pandas-ta` to ensure outputs match Python expectations precisely.

### 3. `tb_bitwise` (SIMD Alpha Generation Engine)
- **Status:** **IMPLEMENTED (Actively Tuning)**
- **Contents:**
  - `precompute.rs`: Generates boolean arrays (bitsets) for indicator conditions. Sparsity and Correlation culling are fully parallelized with `rayon`.
  - `ga.rs` & `engine.rs`: The core genetic algorithm that evolves strategies over multiple generations using logical combinations (AND/OR).
  - `archive.rs`: A MAP-Elites multi-dimensional archive that ensures diversity during generation.
  - `metrics.rs`: Computes Sharpe, Sortino, Profit Factor, Drawdown, Win Rate, CPC Index, and Expectancy without requiring full simulation loops.

### 4. `tb_data` (Data Ingestion)
- **Status:** **IMPLEMENTED**
- **Contents:**
  - `RawData` struct for loading CSV files into Polars DataFrames and raw `Vec<f64>` arrays for fast processing.

### 5. `tb_foundry` (Legacy / Wrapper Generation)
- **Status:** **DELETED**
- **Contents:**
  - Originally designed for `continuous.rs` (Polars Native generation) and `discrete.rs`. 
  - **Current State:** Completely deleted from the workspace. The slow Polars native generation was completely superseded by `tb_bitwise`.

### 6. `tb_simulator` (Phase 2: Walk-Forward Optimization)
- **Status:** **STUBBED (DO NOT IMPLEMENT YET)**
- **Contents:**
  - Contains folders for `portfolio` and `wfo` (Walk-Forward Optimization).
  - The UI has a tab for it, but the backend is strictly a placeholder. 
  - We are actively ignoring Phase 2 until Phase 1 is fully validated and perfect.

### 7. `tb_ui` (egui Frontend UI)
- **Status:** **PARTIALLY IMPLEMENTED (Phase 1 Focused)**
- **Contents:**
  - **Data Sandbox Tab:** Implemented. Can load CSVs into memory.
  - **Alpha Foundry Tab:** Implemented. Allows Phase 1 configuration (Target/Exits, Starting Equity, Indicator Universe). Triggers the `tb_bitwise` evolution in a background thread and displays a dynamic loading spinner followed by the MAP-Elites Leaderboard grid.
  - **Simulator Tab:** Stubbed. Contains UI elements but is not currently wired to an active Phase 2 backend.

---

## Recent Major Decisions
1. **Moved to Bitwise SIMD:** We abandoned Polars-native `Expr` generation for Phase 1 because it was too slow. We now precompute boolean matrices in `tb_bitwise`.
2. **Parallelization:** Rayon is heavily utilized for culling loops to prevent UI bottlenecking.
3. **Phase 1 Priority:** All configurations impacting initial strategy generation (including `starting_equity` for accurate % returns) belong in `Phase1Config`.

## Next Steps / Active Tasks
- [ ] Run the `tb_math` indicator parity audit against `pandas-ta`.
- [ ] Finalize UI table sorting/filtering for the Alpha Foundry results.
- [ ] Extract an elite strategy from the UI and translate it into code (MQL5 or Python) via the AST.
