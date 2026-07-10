# Greenfield Project: Vibe Coding Master Instructions

When generating, modifying, or refactoring code for the `greenfield` multi-crate workspace, you MUST adhere to the following architectural laws. This project is optimized for AI generation, meaning strict boundaries and simplified memory management are paramount.

## 1. Zero-Lifetime Policy
* **Rule:** Do NOT use Rust lifetimes (`<'a>`) unless it is a mathematical impossibility to avoid them.
* **Why:** Lifetimes cause AI context hallucination across large workspaces.
* **Solution:** Pass data by value (ownership transfer) or use `std::sync::Arc` for shared read-only state. Cloning cheap configurations or `Arc` pointers is preferred over borrowing.

## 2. Strict Crate Boundaries
* **Rule:** The project is a Cargo Workspace. Code must be strictly siloed into the appropriate micro-crate (`tb_core`, `tb_data`, `tb_foundry`, `tb_simulator`, `tb_ui`). 
* **Rule:** Crates must NEVER depend on each other horizontally (e.g., `tb_foundry` cannot depend on `tb_simulator`). All cross-communication must happen via traits defined in `tb_core`.

## 3. Data-Oriented Concurrency (Channels)
* **Rule:** Do NOT use `Arc<Mutex<T>>` for cross-thread synchronization. 
* **Solution:** Use **Crossbeam Channels** (`crossbeam_channel::unbounded`). The pipeline is a Producer-Consumer model. Thread A does work and pushes the owned result into a channel. Thread B listens to the channel. Data flows strictly in one direction.

## 4. Polars over Ndarray (for Phase 1)
* **Rule:** When writing matrix/array math for Phase 1 Alpha Generation, use 100% **Polars Native Expressions** (`Expr`). 
* **Why:** It avoids the `unsafe` memory conversions required by `ndarray`, guaranteeing Vibe-Coded stability, and automatically unlocks cuDF GPU acceleration.

## 5. JSON AST Enforcement
* **Rule:** All strategy logic (sketches) must be represented as the `tb_core::Sketch` JSON AST (Recursive Internally Tagged Enums). Rust should NEVER do string-concatenation to generate code (e.g., writing `.mq5` files). Rust only parses the JSON; the LLM API does the translation.

## 6. Error Handling & Logging
* **Rule:** Use `thiserror` for library crates (`tb_core`, `tb_data`) to expose strict enums. Use `anyhow` for application boundaries and the UI.
* **Rule:** Use the `tracing` crate for all logging instead of `println!`. This ensures we can easily pipe logs to files or UI consoles later.

## 7. Testing Strategy
* **Rule:** Always write `cargo test` modules at the bottom of the file you are modifying. 
* **Focus:** Prioritize unit testing `tb_core` (AST serialization round-trips) and integration testing `tb_foundry` (ensuring a known AST produces the expected Polars target matrix).
