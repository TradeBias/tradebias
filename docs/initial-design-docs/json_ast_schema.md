# The JSON AST Schema (`tb_core::Sketch`)

> **Context:** Because we are using an LLM to translate winning strategies into `MQL5/NinjaScript`, the bridge between the Rust engine and the AI is a strict JSON Abstract Syntax Tree (AST). Rust must safely generate and serialize this JSON, and the AI must reliably parse it for codegen.

This document outlines the schema design that makes this bridge bulletproof.

---

## 1. The Design Philosophy (Recursive Enums)

The AST must be recursive. A trading signal is essentially a tree of expressions evaluating to a `bool`. 
To make it perfectly parseable by both AI and Rust, we use an **Internally Tagged Enum** structure. Every JSON object has a `"type"` field that tells Rust exactly what node it is.

---

## 2. The Rust Definition (`tb_core`)

Here is how the AST is defined in Rust. Notice how the `Box<Expr>` allows indicators to be nested infinitely deep (e.g., an SMA of an RSI).

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")] // This tells serde to look for the "type" field in the JSON
pub enum Expr {
    // 1. Data Sources (Leaves)
    Close,
    Open,
    High,
    Low,
    Volume,
    
    // 2. Constants (Leaves)
    Constant { value: f64 },

    // 3. Indicators (Nodes)
    Sma { source: Box<Expr>, period: u32 },
    Ema { source: Box<Expr>, period: u32 },
    Rsi { source: Box<Expr>, period: u32 },
    Macd { source: Box<Expr>, fast: u32, slow: u32, signal: u32 },
    Atr { period: u32 },

    // 4. Mathematical Operators
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    
    // 5. Logical Operators (Return Booleans)
    CrossAbove { lhs: Box<Expr>, rhs: Box<Expr> },
    CrossBelow { lhs: Box<Expr>, rhs: Box<Expr> },
    GreaterThan { lhs: Box<Expr>, rhs: Box<Expr> },
    LessThan { lhs: Box<Expr>, rhs: Box<Expr> },
    
    // 6. Conjunctions (Combine Booleans)
    And { lhs: Box<Expr>, rhs: Box<Expr> },
    Or { lhs: Box<Expr>, rhs: Box<Expr> },
}

/// A Strategy Sketch is just an entry condition (an Expr that evaluates to a boolean)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sketch {
    pub name: String,
    pub entry_long: Expr,
    pub entry_short: Option<Expr>, // If None, we just invert the long logic
}
```

---

## 3. The JSON Output (Serialized by Rust)

When Rust's procedural generation engine discovers a winning strategy (e.g., going long when the 10 SMA crosses above the 50 SMA with an RSI filter), it serializes it into this perfectly structured JSON:

```json
{
  "name": "SMA_Cross_with_RSI_Filter",
  "entry_long": {
    "type": "And",
    "lhs": {
      "type": "CrossAbove",
      "lhs": {
        "type": "Sma",
        "source": { "type": "Close" },
        "period": 10
      },
      "rhs": {
        "type": "Sma",
        "source": { "type": "Close" },
        "period": 50
      }
    },
    "rhs": {
      "type": "LessThan",
      "lhs": {
        "type": "Rsi",
        "source": { "type": "Close" },
        "period": 14
      },
      "rhs": {
        "type": "Constant",
        "value": 30.0
      }
    }
  }
}
```

---

## 4. Why This Schema is "AI-Proof"

1. **Self-Documenting for the LLM:** Because the JSON schema relies heavily on the `"type"` field (internally tagged), it explicitly forces the LLM to declare what node it is writing. LLMs rarely hallucinate when following a strict `"type": "..."` pattern.
2. **Infinite Composability:** The user can ask for absurd logic like: *"SMA of the RSI crossing above the EMA of the MACD."* Because `source` takes an `Expr`, the JSON gracefully nests without any changes to the schema.
3. **Easy Phase 1 Translation:** When this JSON is deserialized by Rust, Phase 1 simply walks the tree and converts it into a `Polars Expr`. 
   * A `CrossAbove` node in Rust trivially maps to Polars: `lhs.shift(1) < rhs.shift(1) & lhs > rhs`.
4. **Easy Translation to MQL5:** When Phase 2 finishes and passes this JSON back to the LLM to write the Expert Advisor, the LLM reads the exact same JSON it generated. The nested tree structure makes it trivial for the LLM to write the corresponding `iMA` and `iRSI` MQL5 handles.

---

## 5. The Meta-Configuration Schema

While the AST above handles the *entry logic* (generated procedurally by Rust), the overall pipeline still needs to be configured based on the user's intent. 

If the user types a fuzzy prompt in the UI like:
> *"Run a backtest using scalping tactics. Set a super tight stop loss, and I only want home run trades that work in bull markets."*

The UI can use a lightweight LLM API call to parse this intent and output a `SessionConfig` JSON object. This maps the user's natural language to the strict backend abstractions we defined in `alpha_target_engineering.md` and `core_pipeline_architecture.md`.

### The Rust Definition (`SessionConfig`)

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionConfig {
    pub phase1: Phase1Config,
    pub phase2: Phase2Config,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase1Config {
    /// Maps to "super tight stop loss" -> "Scalping"
    pub trading_style: TradingStyle, 
    /// Maps to "Home run trades" -> "MaximizeWinLoss"
    pub optimization_focus: OptimizationFocus, 
    pub risk_appetite: RiskAppetite,
    pub complexity_cap: ComplexityCap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase2Config {
    pub exit_strategy: ExitStrategy,
    pub position_sizing: PositionSizing,
    pub frictions: Frictions,
}

// Sensible Defaults for Vibe Coding
impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            phase1: Phase1Config {
                trading_style: TradingStyle::Swing,
                optimization_focus: OptimizationFocus::MaximizeEdge,
                risk_appetite: RiskAppetite::Conservative,
                complexity_cap: ComplexityCap::HumanReadable,
            },
            phase2: Phase2Config {
                exit_strategy: ExitStrategy::TrailingStop,
                position_sizing: PositionSizing::Fixed1Percent,
                frictions: Frictions::Low,
            }
        }
    }
}
```

### The JSON Output

Based on the prompt above, the AI translates the fuzzy language into this strict config:

```json
{
  "phase1": {
    "trading_style": "Scalping",
    "optimization_focus": "MaximizeWinLoss",
    "risk_appetite": "Aggressive",
    "complexity_cap": "HumanReadable"
  },
  "phase2": {
    "exit_strategy": "TrailingStop",
    "position_sizing": "VolatilityAdjusted",
    "frictions": "Low"
  }
}
```

By providing this schema to the LLM, we guarantee that **any** fuzzy language the user throws at it (e.g., "tight stops", "huge winners", "safe trades") is mathematically forced into our predefined, safe pipeline boundaries. The Rust backend receives this JSON and instantly configures the Virtual TP/SL ratio and the Pareto sorting objectives!
