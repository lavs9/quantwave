# Kinematic Kalman Filter

<div class="indicator-meta"><span class="category-badge">ML Features</span> <span class="kw-badge">kalman</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">kinematic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">lag-reduction</span></div>

A second-order Kalman filter that tracks both price (position) and momentum (velocity). By modeling price as a dynamic system with velocity, this filter significantly reduces lag during trending markets compared to standard moving averages or 1D filters.

## Parameters

| Name | Default | Description |
|------|---------|-------------|
| `q_pos` | 0.001 | Process noise for position. Controls sensitivity to price level changes. |
| `q_vel` | 0.0001 | Process noise for velocity. Controls sensitivity to momentum changes. |
| `r` | 0.1 | Measurement noise. Higher values increase smoothing but add lag. |

## Details

The Kinematic Kalman Filter extends the foundational state-space approach by incorporating a velocity state. This allows the filter to "anticipate" the next price based on current momentum, providing a zero-lag-like response during strong trends while maintaining optimal smoothness.

In each recursive step, it updates a 2D state vector $[x, v]^T$ (position and velocity) and a 2x2 covariance matrix. This represents the system's optimal estimate of the "true" price and current momentum, balancing new price data against its internal kinematic model.

## Formula

The implementation follows the recursive matrix equations for a linear discrete Kalman filter:

\[
\hat{x}_{k|k-1} = \Phi \hat{x}_{k-1|k-1}
\]
\[
P_{k|k-1} = \Phi P_{k-1|k-1} \Phi^T + Q
\]
\[
K_k = P_{k|k-1} H^T (H P_{k|k-1} H^T + R)^{-1}
\]
\[
\hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k (z_k - H \hat{x}_{k|k-1})
\]
\[
P_{k|k} = (I - K_k H) P_{k|k-1}
\]

Where $\Phi$ is the state transition matrix $\begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix}$ and $H$ is the measurement matrix $[1, 0]$.

## Source

- **R.E. Kalman (1960):** ["A New Approach to Linear Filtering and Prediction Problems"](https://www.cs.unc.edu/~welch/kalman/media/pdf/Kalman1960.pdf)
