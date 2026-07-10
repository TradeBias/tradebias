# AI Integration Architecture (The Output Codegen Paradigm)

> **Context:** In the legacy project, the Rust backend included massive, brittle code-generation blocks to transpile AST logic into MQL5 and NinjaScript. In the Greenfield project, we are completely removing these hardcoded transpilers. We are replacing them with an "AI API" that acts as the final code translator (Codegen).

This document outlines the feasibility, architecture, and exact flow of how an LLM (Large Language Model) interacts with the Rust backend.

---

## 1. The Core Philosophy (Rust does Math & Generation, AI does Translation)

The fundamental design shift is: **Rust should never try to write MQL5 code.** 
Rust's job is high-speed data processing, procedurally generating strategy logic (ASTs), matrix multiplication, and Genetic Algorithm (GA) execution. 
The LLM's job is exclusively taking the final winning JSON AST and translating it to Trading Platform Code (MQL5/Pine/Ninja).

*Note: Initially, we considered using the LLM API to generate the initial seed population of strategies based on user prompts. We abandoned this because procedural generation within Rust is infinitely faster, cheaper, and more reliable than hitting an LLM API. The LLM is reserved purely for the final output.*

### The "JSON Handshake"
The bridge between the Rust backend and the AI is a strictly typed JSON Abstract Syntax Tree (AST). Rust serializes its winning strategy to JSON; the AI translates that JSON.

## 2. Output Translation (Rust -> AI -> MQL5/NinjaScript)

Once Phase 1 and Phase 2 finish, the system produces a winning "Tear Sheet" based on an elite JSON AST. We now need to deploy this strategy to a broker.

### The Workflow (RAG Template Translation)
Instead of Rust concatenating strings to build an `.mq5` file, we ask the LLM API to do the translation.

1. **The Retrieval-Augmented Generation (RAG) Context:** The Rust backend fetches the winning `EliteStrategy` JSON. It also loads a perfectly compiling "Blank Base Template" for the requested platform (e.g., `BaseTemplate.mq5` which handles all standard order execution and risk management).
2. **The Translation Prompt:** Rust sends the API prompt:
   > *"You are an expert MQL5 developer. Here is the trading logic in JSON format: [JSON AST]. Insert the required indicator calculations and entry logic into the `OnTick()` function of this attached `BaseTemplate.mq5` file."*
3. **The Output:** The LLM returns the fully functioning `.mq5` or `.cs` file, ready to compile in MetaTrader or NinjaTrader.

### Feasibility: High (With RAG Mitigation)
* **The Risk:** LLMs can occasionally hallucinate syntax errors (e.g., forgetting a semicolon) or use deprecated trading API functions if asked to write an Expert Advisor from scratch.
* **The Solution:** By providing the rigid `BaseTemplate.mq5`, we heavily restrict the AI's ability to hallucinate. It only has to fill in the blank `OnTick()` logic block, leveraging the LLM's strength (logic translation) while removing its weakness (boilerplate scaffolding).

---

## 3. Why This Must Be Designed Early

If we did not establish this AI Integration Architecture on Day 1, we would make a catastrophic engineering mistake: **We would accidentally start building an MQL5 transpiler in Rust.**

By establishing that the pipeline boundary ends at the JSON AST, we save thousands of lines of code. It perfectly isolates `tb_core` as a purely mathematical engine. Furthermore, it gives the product an infinite shelf life: if a user wants to export to a brand new trading platform (like TradeStation), we don't have to write a Rust transpiler for it—we just give the LLM API a TradeStation Base Template. 

This is the ultimate modular separation of concerns.
