# UI Architecture & User Experience (egui)

> **Context:** This document outlines the user interface design for the greenfield algorithmic trading system. The goal is to translate complex quantitative mathematics (GAs, CPCV, Pareto sorting) into an intuitive, gamified, and highly visual retail experience.

---

## 1. The Technology Stack

### Core Framework: `egui`
The entire user interface will be built using [egui](https://github.com/emilk/egui), an immediate mode GUI in Rust.
* **Why egui?** It is blazing fast, perfectly integrates with our asynchronous Rust backend (Thread A/B), and can be compiled natively for desktop or to WebAssembly (WASM) for web deployment.

### Charting Engine: `egui-charts`
All data visualization will be powered by [egui-charts](https://github.com/userFRM/egui-charts).
* **Why egui-charts?** Standard egui plots are too basic for financial data. `egui-charts` is specifically designed for rendering high-performance financial charts (OHLC/Candlesticks, Volume, and Equity Curves). It handles zooming, panning, and rendering thousands of candles without dropping frames.

### Centralized Theming (Shadcn Aesthetics)
To prevent `egui` from looking like a default developer tool, the UI must implement a **Centralized Theme Config**.
* **The Goal:** Emulate the sleek, modern aesthetics of **Shadcn UI** (popular in web dev).
* **The Rules:** We will define a strict `theme.rs` that applies absolute values for `egui::Style` across the entire app. This means sleek dark-mode slate grays, minimal rounding (e.g., 4px radius), 1px subtle borders, high-contrast text, and a modern sans-serif font (like Inter or Roboto). No chaotic inline coloring allowed.

---

## 2. The User Journey (Screen by Screen)

The UI is designed as a linear workflow that directly maps to our backend pipeline.

### Screen 1: The Custom Sandbox (Data & Regime Selection)
**The Goal:** Let the user visually define the world they want their strategy to survive in.
* **The View:** A massive, interactive candlestick chart of the selected asset (rendered via `egui-charts`).
* **The Interaction:** The user uses their mouse to click and drag over the chart to highlight specific periods of time (e.g., highlighting the 2020 crash in red, and the 2021 bull run in green).
* **Backend Hook:** These highlighted timestamp ranges are stitched together to form the "Master Dataset" that Phase 1 will apply CPCV across.

### Screen 2: The Alpha Foundry (Phase 1 Config)
**The Goal:** Configure the *personality* of the Genetic Algorithm.
* **The View:** A sleek sidebar for settings, with the main panel showing a live, real-time visualization of the GA's progress.
* **The Core Controls (Left Sidebar):**
  * *Trading Style:* Slider (Scalping <---> Position Trading)
  * *Risk Appetite:* Slider (Conservative <---> Aggressive)
  * *Optimization Focus:* Dropdown (Simplicity vs Downside Protection vs Home Run Trades)
  * *Complexity Cap:* Toggle (Human Readable vs Complex Logic)
* **Live GA Execution Controls:**
  * **Progress Reporting:** Clearly displays the current state of the backend loop (e.g., `"Generation 142 of 500 (28%)"`).
  * **Action Buttons:** The user can **Pause** the GA mid-flight, or hit **Stop/Cancel** to immediately terminate Phase 1 and push the *current* Pareto frontier straight to Phase 2.
* **The Live Feed (Main Panel):** As the asynchronous "Producer" thread generates Elite (Rank 0) strategies, they stream into a live scatter plot. The user watches in real-time as the GA explores the fitness landscape.

### Screen 3: The Execution Simulator (Phase 2 Config)
**The Goal:** Configure how the strategies are managed in the real world.
* **The Controls:**
  * *Position Sizing:* Fixed 1% vs Volatility-Adjusted.
  * *Exit Management:* Hard Take-Profit vs ATR Trailing Stop.
  * *Broker Frictions:* Low (Forex) vs High (Crypto/Altcoins).
  * *Pain Threshold:* Reject if WFO Drawdown > X%.

### Screen 4: The Strategy Tear Sheet (Final Output)
**The Goal:** Present the surviving elite strategies in an institutional-grade report.
* **The View:** A split-pane view. 
  * *Top Half:* A massive, interactive Equity Curve charting the strategy's real-world Walk-Forward Optimization (WFO) performance (rendered via `egui-charts`).
  * *Bottom Half:* The "Alpha Integrity Audit" metrics. 
    * Displays Phase 2 execution stats (CAGR, Max Drawdown, Win Rate).
    * Displays Phase 1 mathematical stats (CPCV Variance score, ensuring the edge is stationary).
* **The Interaction:** The user can click on any individual trade on the equity curve, and the UI will snap the candlestick chart to that exact moment in time, overlaying the AST indicators so the user can visually verify exactly *why* the AI took the trade.

---

## 3. UI/Backend Asynchronous Flow

Because `egui` is an immediate mode GUI, it redraws the screen at 60 FPS. This makes it the perfect match for our **Asynchronous Producer-Consumer** backend.

1. The `egui` thread simply listens to a `crossbeam_channel::Receiver`.
2. The background worker threads (Thread Pool B) silently run Phase 2 WFO simulations.
3. Every time a worker finishes a strategy, it pushes the Tear Sheet data into the channel.
4. On the next frame (1/60th of a second later), `egui` sees the new data in the receiver and instantly updates the UI with the new strategy. 

This guarantees the UI never freezes, never shows a spinning loading wheel, and provides a continuous, highly gamified stream of dopamine for the retail user as they watch the AI discover alpha in real-time.
