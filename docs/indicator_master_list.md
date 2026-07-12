# Master Indicator Blueprint

This document outlines **127** distinct technical indicators that can be modularly implemented in `tb_math` and registered into the Bitwise Engine. They are organized by category to help prioritize which mathematical models to build first.

## 📈 Trend & Moving Averages
Moving averages and trend-following bands used to determine directional bias.
1. **SMA** (Simple Moving Average) - *Implemented*
2. **EMA** (Exponential Moving Average) - *Implemented*
3. **WMA** (Weighted Moving Average) - *Implemented*
4. **HMA** (Hull Moving Average) - *Implemented*
5. **ALMA** (Arnaud Legoux Moving Average) - *Implemented*
6. **ZLEMA** (Zero Lag Exponential Moving Average)
7. **TEMA** (Triple Exponential Moving Average) - *Implemented*
8. **DEMA** (Double Exponential Moving Average) - *Implemented*
9. **KAMA** (Kaufman Adaptive Moving Average) - *Implemented*
10. **VWMA** (Volume Weighted Moving Average)
11. **SMMA** (Smoothed Moving Average) - *Implemented*
12. **TMA** (Triangular Moving Average)
13. **FRAMA** (Fractal Adaptive Moving Average)
14. **VIDYA** (Volatility Index Dynamic Average)
15. **JMA** (Jurik Moving Average)
16. **McGinley Dynamic**
17. **Ichimoku Cloud** (Tenkan, Kijun, Senkou A/B, Chikou)
18. **Parabolic SAR** (Stop and Reverse) - *Implemented*
19. **Supertrend** - *Implemented*
20. **Keltner Channels** - *Implemented*
21. **Donchian Channels** - *Implemented*
22. **Alligator Indicator** (Jaw, Teeth, Lips)
23. **Guppy Multiple Moving Average** (GMMA)
24. **T3** (Tillson Moving Average)
25. **Trend Magic**

## 🌊 Momentum & Oscillators
Bounded and unbounded oscillators used to detect overbought/oversold conditions and divergences.
26. **RSI** (Relative Strength Index) - *Implemented*
27. **MACD** (Moving Average Convergence Divergence) - *Implemented*
28. **Stochastic Oscillator** - *Implemented*
29. **Stochastic RSI** - *Implemented*
30. **Williams %R** - *Implemented*
31. **CCI** (Commodity Channel Index) - *Implemented*
32. **MFI** (Money Flow Index) - *Implemented*
33. **ROC** (Rate of Change) - *Implemented*
34. **Awesome Oscillator** (AO) - *Implemented*
35. **TSI** (True Strength Index) - *Implemented*
36. **UO** (Ultimate Oscillator) - *Implemented*
37. **DPO** (Detrended Price Oscillator) - *Implemented*
38. **KST** (Know Sure Thing) - *Implemented*
39. **Fisher Transform** - *Implemented*
40. **Connors RSI** - *Implemented*
41. **Chande Momentum Oscillator** (CMO) - *Implemented*
42. **RVI** (Relative Vigor Index) - *Implemented*
43. **SMI** (Stochastic Momentum Index) - *Implemented*
44. **TRIX** (Triple Exponential Average Oscillator) - *Implemented*
45. **EOM** (Ease of Movement) - *Implemented*
46. **VORTEX Indicator** - *Implemented*
47. **Bressert Double Smoothed Stochastic** - *Implemented*
48. **PPO** (Percentage Price Oscillator) - *Implemented*
49. **DMI / ADX** (Directional Movement Index) - *Implemented*
50. **Choppiness Index** - *Implemented*
51. **QQE** (Quantitative Qualitative Estimation) - *Implemented*
52. **Schaff Trend Cycle** (STC) - *Implemented*

## ⚡ Volatility
Indicators that measure the rate of price fluctuations, useful for dynamic stops and breakouts.
53. **Bollinger Bands** - *Implemented*
54. **ATR** (Average True Range) - *Implemented*
55. **Chaikin Volatility** - *Implemented*
56. **Historical Volatility** (HV) - *Implemented*
57. **Ulcer Index** - *Implemented*
58. **Standard Deviation** (StdDev Rolling Window) - *Implemented*
59. **Bollinger Band Width** - *Implemented*
60. **Bollinger %B** - *Implemented*
61. **Keltner Channel Width** - *Implemented*
62. **VIX Synthetic** (Price-implied volatility models) - *Implemented*

## 📊 Volume & Order Flow
Indicators that require trading volume arrays to confirm price movements.
64. **OBV** (On-Balance Volume) - *Implemented*
65. **VWAP** (Volume Weighted Average Price) - *Implemented*
66. **Accumulation/Distribution Line** (A/D) - *Implemented*
67. **Chaikin Money Flow** (CMF) - *Implemented*
68. **Chaikin Oscillator** - *Implemented*
69. **PVT** (Price Volume Trend) - *Implemented*
70. **NVI** (Negative Volume Index) - *Implemented*
71. **PVI** (Positive Volume Index) - *Implemented*
72. **Force Index** - *Implemented*
73. **VFI** (Volume Flow Indicator) - *Implemented*
74. **Volume Oscillator** (VOSC) - *Implemented*
75. **Klinger Oscillator** - *Implemented*
76. **MVWAP** (Moving VWAP) - *Implemented*
77. **TWAP** (Time Weighted Average Price) - *Implemented*

## 📐 Statistical & Mathematical
Pure mathematical models applied to time series arrays.
78. **Linear Regression Slope** - *Implemented*
79. **Linear Regression Intercept** - *Implemented*
80. **Linear Regression R-Squared** - *Implemented*
81. **Linear Regression Curve** - *Implemented*
82. **Standard Error Bands** - *Implemented*
83. **Z-Score** (Standardized Price) - *Implemented*
84. **Log Return** - *Implemented*
85. **Median Price** ((High + Low) / 2) - *Implemented*
86. **Typical Price** ((High + Low + Close) / 3) - *Implemented*
87. **Weighted Close** ((High + Low + Close * 2) / 4) - *Implemented*
88. **Hurst Exponent** (Fractal predictability) - *Implemented*
89. **Pivot Points** (Standard, Fibonacci, Woodie, Camarilla) - *Implemented*
90. **Fibonacci Retracement Levels** - *Implemented*
91. **Heikin-Ashi** (Synthetic OHLC conversion) - *Implemented*

## 🧠 Advanced DSP (Digital Signal Processing)
Highly advanced models (often pioneered by John Ehlers) utilizing digital signal processing for zero-lag filtering.
93. **Ehlers Super Smoother Filter** - *Implemented*
94. **Ehlers Decycler** - *Implemented*
95. **Ehlers Cyber Cycle** - *Implemented*
96. **Ehlers MESA Adaptive Moving Average** (MAMA) - *Implemented*
97. **Ehlers FAMA** (Following Adaptive Moving Average) - *Implemented*
98. **Sine Wave Indicator** (Hilbert Transform) - *Implemented*
99. **Autocorrelation Periodogram** - *Implemented*
100. **Dominant Cycle Period** - *Implemented*
101. **Decycler Oscillator** - *Implemented*
102. **Empirical Mode Decomposition (EMD)** - *Implemented*
103. **Market Meanness Index (MMI)** - *Implemented*
104. **Roofing Filter** - *Implemented*
105. **Zero-Lag MACD** - *Implemented*
106. **Gator Oscillator** - *Implemented*
107. **Kalman Filter** (1D Price State Estimation) - *Implemented*

## 🕯️ Candlestick Patterns
Binary pattern recognition arrays returning 1.0 (True) or 0.0 (False).
108. **Bullish Engulfing**
109. **Bearish Engulfing**
110. **Doji**
111. **Hammer**
112. **Shooting Star**
113. **Morning Star**
114. **Evening Star**
115. **Piercing Line**
116. **Dark Cloud Cover**
117. **Bullish Harami**
118. **Bearish Harami**
119. **Three White Soldiers**
120. **Three Black Crows**
121. **Inverted Hammer**
122. **Hanging Man**
123. **Marubozu** (Bullish/Bearish)
124. **Spinning Top**
125. **Tweezer Bottom**
126. **Tweezer Top**
