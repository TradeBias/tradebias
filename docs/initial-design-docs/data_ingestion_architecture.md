# Data Ingestion & Storage Architecture

> **Context:** A backtesting engine is only as good as the data it processes. Because we are using Polars to evaluate 100,000 strategies simultaneously in Phase 1, our data must be perfectly standardized, lightning-fast to load, and correctly aligned.

This document outlines how data flows from the outside world (APIs, CSVs) into the strict `tb_data` micro-crate format.

---

## 1. The Ingestion Sources

To provide maximum flexibility to the retail user, the system supports three primary data sources:

1. **Consumer APIs (User Keys):**
   * The user provides their own API keys (e.g., Binance, Polygon, Alpaca).
   * **Security:** API keys are encrypted and managed via **Supabase**. The Rust backend securely requests the data stream.
2. **Supabase Curated Datasets:**
   * We (the platform) host ready-made, high-quality datasets on Supabase. The user can just click "Import S&P 500 (2010-2025)" without needing their own API key.
3. **Local CSV Import:**
   * The user can upload their own proprietary CSV files (e.g., exported from MetaTrader or NinjaTrader).

---

## 2. The Standardization Pipeline

The biggest risk in backtesting is malformed data. `tb_data` acts as a strict firewall that sanitizes everything before it reaches the matrix engine.

### A. The Data Mapping Table
Because every CSV and API uses different column names (e.g., `Date`, `datetime`, `Timestamp`, `Open`, `O`, `price`), the UI will present a **Data Mapping Table**.
* The user visually maps their source columns to our strict internal schema: `[timestamp, open, high, low, close, volume]`.
* This eliminates the risk of the matrix math failing due to missing columns.

### B. DateTime Handling
Timezones and weird timestamp formats (e.g., Unix milliseconds vs `YYYY-MM-DD HH:MM:SS`) cause massive alignment bugs.
* `tb_data` leverages Rust's `chrono` and `polars` time-series parsing. 
* All incoming datetime strings/ints are strictly parsed, stripped of timezone ambiguity, and cast to a standard UTC datetime array.

---

## 3. The Storage Format (Apache Parquet)

**Rule:** We never run Phase 1 directly on a CSV file.

When data is ingested (whether via API or CSV), it is immediately converted and saved locally as an **Apache Parquet** (`.parquet`) file.

### Why Parquet?
1. **Lightning Fast:** Parquet is the native format for Polars. It is heavily compressed and column-oriented. 
2. **Memory Mapping:** Polars can `LazyLoad` a 10GB Parquet file instantaneously using memory-mapping, meaning it only pulls the specific columns it needs into RAM. This prevents the user's computer from crashing when testing 10 years of tick data.
3. **Caching:** If the user runs 5 different backtests on the "2020-2023 BTC" dataset, we don't fetch it from the API 5 times. We pull it from the local Parquet cache.

---

## 4. Multi-Timeframe (MTF) Alignment

Modern strategies rely on multiple timeframes (e.g., "The Daily trend is up, so I will take the 1-Hour crossover").

If a user requests MTF indicators, `tb_data` is responsible for building the **Master Dataset** *before* Phase 1 begins.
* It computes the Daily indicators on the Daily DataFrame.
* It then uses Polars `join_asof` (or forward-filling) to broadcast those Daily indicator values down onto the 1-Hour execution DataFrame rows.
* By aligning all timeframes into a single, flat Parquet table, Phase 1 matrix math remains simple, vectorized, and blistering fast.
