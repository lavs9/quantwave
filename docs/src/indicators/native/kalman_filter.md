# Kalman Filter

<div class="indicator-meta"><span class="category-badge">ML Features</span> <span class="kw-badge">filter</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">ml</span> <span class="kw-badge">kalman</span></div>

An adaptive 1D Kalman filter for smoothing price data with minimal lag.

## Usage

Use as a highly responsive alternative to moving averages. The Q parameter (process noise) controls responsiveness to trend changes, while R (measurement noise) controls smoothness. Higher Q makes it track price faster; higher R increases smoothing.

## Background

> The Kalman Filter is an optimal estimator for linear systems with Gaussian noise. In technical analysis, the 1D version recursively updates the estimate of the 'true' price by balancing the predicted state against new measurements. It is particularly effective for feature engineering in ML models due to its ability to separate signal from noise dynamically.

## Parameters

- `q` (default: 0.01): Process noise (responsiveness)
- `r` (default: 0.1): Measurement noise (smoothing)

## Formula


\[
P_{t|t-1} = P_{t-1} + Q
\]
\[
K_t = \frac{P_{t|t-1}}{P_{t|t-1} + R}
\]
\[
X_t = X_{t-1} + K_t(Z_t - X_{t-1})
\]
\[
P_t = (1 - K_t)P_{t|t-1}
\]


[Source](https://en.wikipedia.org/wiki/Kalman_filter)
