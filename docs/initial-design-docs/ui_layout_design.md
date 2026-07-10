# UI Layout & Wireframe Design (egui)

> **Context:** While `ui_architecture.md` defines the functionality and workflow of the individual screens, this document defines the holistic, spatial layout of the main application window. It provides the strict wireframe blueprint for the `tb_ui` crate to ensure a sleek, modern, and professional aesthetic.

Because we are targeting a "Shadcn" aesthetic, the layout must be minimal, modular, and prioritize data visualization (charts) above all else.

---

## 1. The Macro Application Shell

The main window will be structured using standard `egui` panels, establishing a persistent "App Shell" that surrounds the active content.

### A. The Left Navigation Rail (`egui::SidePanel::left`)
**Width:** ~64px (Collapsed) or ~200px (Expanded)
**Purpose:** Global app navigation.
* A minimal vertical sidebar containing high-contrast, modern icons (e.g., Lucide icons) for the main tabs:
  1. 📊 **Data Sandbox** (Screen 1)
  2. 🧬 **Alpha Foundry** (Screen 2)
  3. ⚔️ **Execution Simulator** (Screen 3)
  4. 📚 **Strategy Library** (Saved Tear Sheets)
* Uses extremely subtle hover effects (e.g., `bg-slate-800` on hover) with no harsh borders.

### B. The Global Status Bar (`egui::TopBottomPanel::bottom`)
**Height:** ~24px
**Purpose:** Persistent visibility into the asynchronous backend pipeline.
* Because the backend uses crossbeam channels (Thread A and Thread B), this footer allows the user to see background activity regardless of what tab they are on.
* **Left Aligned:** Engine Status (e.g., `🟢 Phase 1: Evolving (142/500)` | `🟡 Phase 2: Processing Queue (4)`)
* **Right Aligned:** System Resources (e.g., `RAM: 1.2GB` | `GPU: Ready`)

---

## 2. The Dynamic Screen Layouts

Depending on the active tab selected in the Navigation Rail, the remaining screen real estate is divided intelligently. 

### C. The Central Canvas (`egui::CentralPanel`)
**Purpose:** The absolute maximum amount of screen space must be dedicated here. 
* This is where `egui-charts` renders the massive OHLCV candlesticks, the scatter plots of the Pareto frontier, and the Walk-Forward Equity Curves.
* **Rule:** The Central Panel has NO padding around the edges of the chart. The chart should bleed cleanly to the borders of the adjacent panels for a premium, immersive feel.

### D. The Configuration Drawer (`egui::SidePanel::right`)
**Width:** ~300px (Collapsible)
**Purpose:** Houses all the sliders, dropdowns, and toggles defined in `ui_architecture.md`.
* Instead of cluttering the main view or using ugly floating windows, all Phase 1 and Phase 2 configurations live in a right-aligned sidebar.
* **Collapsible:** The user can click a small chevron `>` to collapse this panel entirely, expanding the Central Canvas chart to take up 100% of the screen.
* **Layout:** Grouped using sleek `egui::CollapsingHeader` widgets (e.g., expanding "Risk Appetite" reveals the sliders).

---

## 3. Thematic & Spatial Rules (The Shadcn Vibe)

To guarantee the UI does not end up looking like a chaotic "programmer art" tool, the AI must enforce these spatial rules when writing the `tb_ui` code:

1. **Hierarchy via Typography, not Borders:** Do not separate UI sections with thick borders or aggressive contrasting background colors. Use typography (e.g., `Heading`, `Subheading`, `Muted Text`) and subtle spacing to denote hierarchy.
2. **Standardized Margins:** All panels (except the Central Canvas charts) must use a standardized inner padding (e.g., `egui::vec2(16.0, 16.0)`). 
3. **Card-Based UI:** When displaying individual strategies in the "Strategy Library", render them as sleek, rounded cards (`egui::Frame::rounding(4.0)`) with a very subtle 1px stroke (e.g., `Color32::from_gray(40)`).
4. **Modal Overlays:** If the app needs to ask the user a blocking question (e.g., "Are you sure you want to delete this dataset?"), use a centered `egui::Window` without a title bar, acting as a modern modal dialog box with a blurred or darkened background overlay.

By explicitly following this wireframe, `tb_ui` will instantly feel like a premium, institutional-grade web application packaged cleanly into a native Rust desktop executable.
