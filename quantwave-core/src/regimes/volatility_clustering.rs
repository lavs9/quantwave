//! Volatility Regimes (Prakash et al. 2021)
//!
//! Source: Prakash, A., James, N., Menzies, M., & Francis, G. (2021). 
//! "Structural clustering of volatility regimes for dynamic trading strategies." 
//! Applied Mathematical Finance, 28(3), 236-274.
//!
//! Implementation of structural clustering of volatility regimes using rolling ATR and online K-Means.
//! This module identifies discrete volatility states (e.g., Low, Medium, High) by clustering
//! recent ATR values.

use std::collections::VecDeque;
use crate::indicators::volatility::ATR;
use crate::traits::Next;
use crate::regimes::MarketRegime;

/// A streaming volatility clusterer that identifies market regimes based on ATR.
#[derive(Debug, Clone)]
pub struct VolatilityClusterer {
    atr: ATR,
    window: VecDeque<f64>,
    window_size: usize,
    k: usize,
    centroids: Vec<f64>,
    counts: Vec<usize>,
    min_observations: usize,
}

impl VolatilityClusterer {
    /// Creates a new VolatilityClusterer.
    ///
    /// # Arguments
    /// * `atr_period` - The period for the underlying ATR indicator.
    /// * `window_size` - The number of recent ATR values to consider for clustering.
    /// * `k` - The number of volatility regimes to identify (e.g., 3 for Low/Med/High).
    pub fn new(atr_period: usize, window_size: usize, k: usize) -> Self {
        Self {
            atr: ATR::new(atr_period),
            window: VecDeque::with_capacity(window_size),
            window_size,
            k,
            centroids: vec![0.0; k],
            counts: vec![0; k],
            min_observations: window_size.max(k * 10), // Need enough data to stabilize
        }
    }

    /// Performs online K-Means update for a single value.
    fn update_clusters(&mut self, val: f64) -> usize {
        if self.centroids.iter().all(|&c| c == 0.0) {
            // Initialization: spread centroids across current range
            if self.window.len() >= self.k {
                let mut sorted: Vec<f64> = self.window.iter().copied().collect();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                for i in 0..self.k {
                    let idx = (i * (sorted.len() - 1)) / (self.k - 1);
                    self.centroids[i] = sorted[idx];
                    self.counts[i] = 1;
                }
            }
        }

        // Find nearest centroid
        let mut min_dist = f64::MAX;
        let mut closest_idx = 0;

        for (i, &centroid) in self.centroids.iter().enumerate() {
            let dist = (val - centroid).abs();
            if dist < min_dist {
                min_dist = dist;
                closest_idx = i;
            }
        }

        // Online update of centroid
        self.counts[closest_idx] += 1;
        let lr = 1.0 / (self.counts[closest_idx] as f64).sqrt(); // Decaying learning rate
        self.centroids[closest_idx] += lr * (val - self.centroids[closest_idx]);

        closest_idx
    }
}

impl Next<(f64, f64, f64)> for VolatilityClusterer {
    type Output = MarketRegime;

    fn next(&mut self, input: (f64, f64, f64)) -> Self::Output {
        let atr_val = self.atr.next(input);
        
        if self.window.len() >= self.window_size {
            self.window.pop_front();
        }
        self.window.push_back(atr_val);

        let cluster_idx = self.update_clusters(atr_val);

        if self.window.len() < self.min_observations {
            return MarketRegime::Steady;
        }

        // Map cluster index to MarketRegime
        // We sort centroids to ensure 0 is Low, k-1 is High
        let mut sorted_indices: Vec<usize> = (0..self.k).collect();
        sorted_indices.sort_by(|&a, &b| self.centroids[a].partial_cmp(&self.centroids[b]).unwrap());

        let rank = sorted_indices.iter().position(|&i| i == cluster_idx).unwrap_or(0);

        match rank {
            0 => MarketRegime::Steady, // Lowest vol
            r if r == self.k - 1 => MarketRegime::Crisis, // Highest vol
            _ => MarketRegime::Cluster(rank as u8),
        }
    }
}
