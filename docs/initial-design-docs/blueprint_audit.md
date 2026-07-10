# Greenfield Blueprint Audit — PM/SE Review

> **Reviewer:** Project Manager / Software Engineer  
> **Scope:** All 8 documents in `docs/refactor/greenfield/`  
> **Date:** 2026-07-08  
> **Purpose:** Determine whether these docs are comprehensive enough to hand to a fresh AI agent in a brand-new Git repository and say "Build this." Identify conflicts, gaps, ambiguities, and missing features.

---

## 1. Documents Reviewed

| # | Document | Purpose | Size |
|---|----------|---------|------|
| 1 | `AGENTS.md` | AI coding constraints (lifetime, crate, channel rules) | 2 KB |
| 2 | `holistic_system_design.md` | Multi-crate workspace layout, trait boundaries, concurrency | 6 KB |
| 3 | `core_pipeline_architecture.md` | Two-phase pipeline (Alpha Foundry → Execution Simulator) | 9 KB |
| 4 | `alpha_target_engineering.md` | Fitness functions, CPCV, Pareto sorting, hard constraints, UI abstractions | 13 KB |
| 5 | `data_ingestion_architecture.md` | API/CSV/Supabase sources, Parquet, column mapping, MTF | 3 KB |
| 6 | `json_ast_schema.md` | Recursive `Expr` enum, `Sketch` struct, `SessionConfig` | 6 KB |
| 7 | `ai_integration_architecture.md` | LLM codegen for MQL5/NinjaScript via RAG templates | 4 KB |
| 8 | `ui_architecture.md` | egui screens, egui-charts, async UI flow | 5 KB |

---

## 2. Cohesiveness Score: 8.5 / 10

The documents tell a coherent story. The data flow is traceable end-to-end:

```
User Data Selection (UI Screen 1)
  → tb_data (Parquet ingestion, MTF alignment)
  → tb_foundry / Phase 1 (Polars matrix, CPCV, Pareto, Hard Constraints)
  → crossbeam_channel
  → tb_simulator / Phase 2 (WFO, stateful execution, Tear Sheet)
  → tb_ui (egui renders Tear Sheet with egui-charts)
  → LLM API (JSON AST → MQL5/NinjaScript codegen)
```

The crate boundaries in `holistic_system_design.md` match the pipeline phases in `core_pipeline_architecture.md`. The UI screens in `ui_architecture.md` map directly to the Safe Configuration Abstractions in `alpha_target_engineering.md`. The `AGENTS.md` rules reinforce the architectural decisions made across all other docs.

---

## 3. Conflicts Found

### CONFLICT 1: Pareto Objective Count — 2 vs 3

> [!WARNING]
> This is the most dangerous conflict in the blueprint. A fresh AI will not know which to implement.

- **`alpha_target_engineering.md` Step 4 (line 95):** States the Pareto sort uses **2 objectives** — Continuous Cumulative Return and CPCV Variance.
- **`alpha_target_engineering.md` Section 8.3 (line 119):** States the GA uses a **strict 3-objective** Pareto sort, with the user controlling the 3rd slot (Complexity, MAE, or Win/Loss Ratio).

**Resolution needed:** Is the Pareto sort 2-objective or 3-objective? The Step 4 blueprint must explicitly include the 3rd dynamic slot, or Section 8.3 must be removed.

### CONFLICT 2: Phase 1 Description — "No TP/SL" vs Virtual TP/SL

- **`core_pipeline_architecture.md` (line 20):** *"There are no stop-losses, no take-profits, and no stateful portfolio tracking."*
- **`core_pipeline_architecture.md` (line 29):** *"Virtual TP/SL: We pre-compute a Virtual Take-Profit / Stop-Loss array based on ATR."*
- **`alpha_target_engineering.md` (line 84):** Confirms Virtual TP/SL is used.

**Resolution needed:** The bold statement on line 20 is misleading. It should clarify that there are no *stateful* stops, but there *is* a pre-computed Virtual TP/SL target array. An AI reading line 20 literally will skip implementing the Virtual TP/SL entirely.

### CONFLICT 3: `holistic_system_design.md` AST vs `json_ast_schema.md` AST

- **`holistic_system_design.md` Section 4 (line 69):** Describes a flat enum: `enum Node { CrossAbove(usize, usize), Threshold(usize, f64) }` — using `usize` index references to pre-computed indicator columns.
- **`json_ast_schema.md` Section 2 (line 25):** Defines a recursive `Box<Expr>` enum: `Sma { source: Box<Expr>, period: u32 }` — a tree structure with nested expressions.

These are **fundamentally different architectures**. The flat-index approach requires a pre-computed indicator registry; the recursive-tree approach is self-contained.

**Resolution needed:** Pick one. The `json_ast_schema.md` recursive approach is more thoroughly documented and aligns with the AI codegen pipeline. The flat enum in `holistic_system_design.md` should be updated to reference the recursive `Expr` enum, or explicitly noted as an internal Phase 1 optimization that the recursive AST compiles down to.

### CONFLICT 4: `ndarray` Reference in CPCV Section

- **`alpha_target_engineering.md` (line 69):** *"This is just slicing the ndarray before the dot product."*
- **`AGENTS.md` Rule 4:** *"Use 100% Polars Native Expressions. Do NOT use ndarray."*

**Resolution needed:** Replace the `ndarray` reference with `Polars DataFrame` to avoid confusing the AI.

---

## 4. Ambiguities & Under-Specified Areas

### AMBIGUITY 1: The Procedural Generation Engine (The Mutation Engine)

No document specifies **how Rust procedurally generates and mutates strategy ASTs**. The docs establish:
- The GA evaluates millions of strategies (Phase 1).
- The AST is a recursive `Expr` enum (JSON AST Schema).
- The LLM is NOT used for generation (AI Integration).

But nowhere does it say:
- How is the initial seed population of `Sketch` objects created?
- What mutation operators exist? (e.g., swap `Sma` for `Ema`, randomize `period`, prune/grow tree branches)
- What crossover operators exist?
- What is the population size? Tournament selection? Elitism rate?

**Impact:** An AI will have to invent the entire GA mutation engine from scratch with zero guidance. This is one of the most complex parts of the system.

**Recommendation:** Create a `ga_engine_design.md` that specifies mutation operators, crossover, selection pressure, and how the `Expr` tree is randomly generated and modified.

### AMBIGUITY 2: The SketchLibrary (Procedural Templates)

`alpha_target_engineering.md` Section 8 references a `SketchLibrary` with "base sketches" (e.g., `trend_cross`) and "confluence composite sketches." No document defines:
- What sketches exist in the library.
- How sketches relate to the recursive `Expr` enum.
- Whether sketches are hardcoded Rust functions or data-driven JSON templates.

**Impact:** The AI won't know what strategies to generate or how to constrain the search space.

### AMBIGUITY 3: Supabase Integration Depth

`data_ingestion_architecture.md` mentions Supabase for API key encryption and curated datasets but doesn't specify:
- Authentication flow (OAuth? Email/password? Anonymous?)
- Whether the Rust backend talks to Supabase directly via REST or through a separate service.
- Database schema for storing user sessions, saved strategies, or API keys.

**Impact:** Low for Phase 1 build (can be deferred), but will cause confusion if the AI tries to implement the full data pipeline.

### AMBIGUITY 4: Indicator Pre-computation Ownership

- `alpha_target_engineering.md` Step 1 says indicators are pre-computed "before the GA starts."
- `data_ingestion_architecture.md` says `tb_data` handles "Polars preprocessing."
- `holistic_system_design.md` says `tb_foundry` handles "AST mutation, Matrix Math."

**Question:** Who computes the indicators? Is it `tb_data` (during dataset preparation) or `tb_foundry` (during GA initialization)? If `tb_data` pre-computes all possible indicators, it needs to know the full indicator catalogue. If `tb_foundry` does it, `tb_data` is simpler but Phase 1 startup is slower.

**Recommendation:** Clarify that `tb_data` provides raw OHLCV + ATR/Virtual TP/SL targets, and `tb_foundry` computes strategy-specific indicators on-demand from the `Expr` AST during evaluation.

---

## 5. Missing Features & Considerations

### MISSING 1: Error Handling & Logging Strategy

No document specifies:
- How errors propagate across crate boundaries (custom error types? `anyhow`? `thiserror`?).
- Logging framework (`tracing`? `log`?).
- How the UI surfaces backend errors to the user.

**Recommendation:** Add an error handling section to `AGENTS.md` or `holistic_system_design.md`. For vibe coding, recommend `anyhow` for applications and `thiserror` for libraries, plus `tracing` for structured logging.

### MISSING 2: Persistence & Session Management

The docs describe a complete pipeline from data → generation → results, but don't address:
- Can the user save a completed backtest session and reload it later?
- Are Tear Sheets persisted to disk (JSON? SQLite? Parquet?)?
- Can the user compare multiple Tear Sheets side-by-side?
- Is there a "Strategy Library" where winning strategies accumulate over time?

**Impact:** Without persistence, every time the user closes the app, all results are lost.

### MISSING 3: Testing Strategy

`AGENTS.md` says "Verify with `cargo test`" but no document specifies:
- What should be tested (unit tests for AST mutation? Integration tests for the pipeline?).
- Whether mock data fixtures are provided.
- How to test the Polars matrix evaluation without a full GA run.

**Recommendation:** Add a testing section that specifies: unit tests for `tb_core` (AST serialization round-trips), integration tests for `tb_foundry` (known-good strategy on known data produces expected fitness), and mock implementations of the `AlphaGenerator` / `ExecutionSimulator` traits.

### MISSING 4: Configuration File & Defaults

The `SessionConfig` struct exists in `json_ast_schema.md`, but no document specifies:
- Default values for all configuration fields.
- Whether configuration is loaded from a TOML/YAML file on startup.
- How the UI pre-populates sliders and dropdowns with sensible defaults.

**Impact:** An AI will pick arbitrary default values (e.g., population size of 10 instead of 100,000).

### MISSING 5: Progress Reporting & Cancellation

The async pipeline streams results to the UI, but:
- Can the user cancel a running GA mid-generation?
- Is there a progress bar showing "Generation 142 of 500"?
- What happens if the user changes settings while Phase 1 is running?

### MISSING 6: Deployment & Distribution

No document addresses:
- How the final binary is built and distributed (installer? portable `.exe`? WASM web app?).
- Minimum system requirements (RAM, CPU cores, GPU optional?).
- Whether the app requires an internet connection (for Supabase/LLM API) or can run fully offline.

---

## 6. Recommendations Summary

### Must Fix Before Build (Blockers)

| # | Issue | Action |
|---|-------|--------|
| C1 | Pareto 2-obj vs 3-obj conflict | Update Step 4 in `alpha_target_engineering.md` to include the 3rd dynamic objective |
| C2 | "No TP/SL" vs Virtual TP/SL | Reword line 20 of `core_pipeline_architecture.md` to clarify "no stateful execution" |
| C3 | Flat enum vs recursive `Expr` | Update `holistic_system_design.md` Section 4 to reference the recursive `Expr` from `json_ast_schema.md` |
| C4 | ndarray reference | Replace "ndarray" with "Polars DataFrame" in `alpha_target_engineering.md` line 69 |
| A1 | Missing mutation engine design | Create `ga_engine_design.md` specifying mutation, crossover, selection, and population management |

### Should Fix (High Priority)

| # | Issue | Action |
|---|-------|--------|
| A2 | SketchLibrary undefined | Define the initial sketch catalogue and how it relates to `Expr` |
| A4 | Indicator computation ownership | Clarify `tb_data` vs `tb_foundry` responsibility |
| M1 | No error handling strategy | Add `anyhow`/`thiserror`/`tracing` guidance to `AGENTS.md` |
| M2 | No persistence/session save | Add a persistence section to `holistic_system_design.md` |
| M4 | No configuration defaults | Define sensible defaults for all `SessionConfig` fields |

### Can Defer (Nice to Have)

| # | Issue | Action |
|---|-------|--------|
| A3 | Supabase integration depth | Defer to implementation phase |
| M3 | Testing strategy | Add testing guidance to `AGENTS.md` |
| M5 | Progress reporting & cancellation | Add to `ui_architecture.md` |
| M6 | Deployment & distribution | Defer to later milestone |

---

## 7. Final Verdict

**Can a fresh AI build this project from these docs alone?** Almost — but not quite.

The mathematical pipeline (data → matrix → Pareto → execution → tear sheet) is exceptionally well-documented. The crate boundaries and concurrency model are clear. The UI screens map perfectly to the backend phases.

The critical gap is the **Genetic Algorithm engine itself** — the mutation operators, crossover logic, population management, and how `Expr` trees are randomly generated. This is the beating heart of the system and it currently has zero documentation. Without it, the AI will either hallucinate a naive implementation or stall asking for clarification.

Fix the 4 conflicts, write the GA engine design doc, and this blueprint is ready for a clean build.
