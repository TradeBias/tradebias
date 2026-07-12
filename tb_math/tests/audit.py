import pandas as pd
import pandas_ta as ta
import numpy as np

# 1. Generate Synthetic Price Data
np.random.seed(42)
days = 1000

# Random Walk for Close
returns = np.random.normal(loc=0.0001, scale=0.01, size=days)
close = 100 * np.cumprod(1 + returns)

# Generate Open, High, Low
open_price = close * (1 + np.random.normal(loc=0, scale=0.005, size=days))
high = np.maximum(open_price, close) * (1 + np.abs(np.random.normal(loc=0, scale=0.005, size=days)))
low = np.minimum(open_price, close) * (1 - np.abs(np.random.normal(loc=0, scale=0.005, size=days)))
volume = np.random.randint(1000, 100000, size=days)

df = pd.DataFrame({
    'open': open_price,
    'high': high,
    'low': low,
    'close': close,
    'volume': volume
})

# 2. Calculate Indicators using pandas-ta
df['sma_14'] = ta.sma(df['close'], length=14)
df['ema_14'] = ta.ema(df['close'], length=14)
df['rsi_14'] = ta.rsi(df['close'], length=14)

macd = ta.macd(df['close'], fast=12, slow=26, signal=9)
df['macd_line'] = macd['MACD_12_26_9']
df['macd_signal'] = macd['MACDs_12_26_9']
df['macd_hist'] = macd['MACDh_12_26_9']

bbands = ta.bbands(df['close'], length=20, std=2)
df['bb_lower'] = bbands['BBL_20_2.0']
df['bb_mid'] = bbands['BBM_20_2.0']
df['bb_upper'] = bbands['BBU_20_2.0']

df['atr_14'] = ta.atr(df['high'], df['low'], df['close'], length=14)

adx = ta.adx(df['high'], df['low'], df['close'], length=14)
df['adx_14'] = adx['ADX_14']
df['plus_di'] = adx['DMP_14']
df['minus_di'] = adx['DMN_14']

supertrend = ta.supertrend(df['high'], df['low'], df['close'], length=7, multiplier=3.0)
df['supertrend'] = supertrend['SUPERT_7_3.0']

# Fill NaNs with a specific value so Rust can parse it (or leave blank)
df.fillna("NaN", inplace=True)

# 3. Save to validation_targets.csv
df.to_csv("validation_targets.csv", index=False)
print("Saved validation_targets.csv")
