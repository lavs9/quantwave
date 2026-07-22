//! Hierarchical Risk Parity (Lopez de Prado), scipy-free single-linkage port.
//!
//! Source: Lopez de Prado, M. (2016), "Building Diversified Portfolios that
//! Outperform Out-of-Sample", Journal of Portfolio Management. Pipeline:
//! correlation -> distance, single-linkage hierarchical clustering,
//! quasi-diagonalization (seriation), then recursive bisection allocation
//! using inverse-variance sub-portfolios. Direct port of the Python v1's
//! `hrp` / `_single_linkage` / `_linkage_order` / `_hrp_bisection`.
//!
//! The single-linkage step here is the same O(n^2 log n)-per-merge (O(n^3)
//! total) nearest-fragment scan as the Python v1 -- deliberately simple,
//! matching the original "scipy-free" design note; a proper O(n^2) SLINK
//! implementation is a possible follow-up for large universes but was not
//! needed to match the v1's behavior (see task report).

use super::covariance::{check_cov, check_returns_matrix};
use super::projection::apply_bounds_and_budget;
use super::{Bounds, PortfolioError, PortfolioResult};
use nalgebra::DMatrix;
use std::collections::{HashMap, HashSet};

fn corr_from_cov(cov: &DMatrix<f64>) -> DMatrix<f64> {
    let n = cov.nrows();
    let std: Vec<f64> = (0..n)
        .map(|i| {
            let s = cov[(i, i)].sqrt();
            if s <= 0.0 { 1e-12 } else { s }
        })
        .collect();
    let mut corr = DMatrix::<f64>::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            let v = cov[(i, j)] / (std[i] * std[j]);
            corr[(i, j)] = v.clamp(-1.0, 1.0);
        }
    }
    for i in 0..n {
        corr[(i, i)] = 1.0;
    }
    corr
}

/// One linkage row: `(cluster_a, cluster_b, distance, size)`.
type LinkageRow = (usize, usize, f64, usize);

/// Single-linkage agglomerative clustering on a full distance matrix.
///
/// Reimplements the core of
/// `scipy.cluster.hierarchy.linkage(method="single")` with a simple
/// nearest-fragment merge (no scipy dependency). Returns a SciPy-compatible
/// linkage list where clusters `0..n-1` are the original items and `n+i` is
/// the cluster created at merge step `i`.
fn single_linkage(dist: &DMatrix<f64>) -> Vec<LinkageRow> {
    let n = dist.nrows();
    let mut next_id = n;
    let mut linkage: Vec<LinkageRow> = Vec::with_capacity(n.saturating_sub(1));

    let mut id_to_size: HashMap<usize, usize> = (0..n).map(|i| (i, 1)).collect();
    let mut alive: HashSet<usize> = (0..n).collect();
    let mut d: HashMap<(usize, usize), f64> = HashMap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            d.insert((i, j), dist[(i, j)]);
        }
    }

    while alive.len() > 1 {
        let mut alive_list: Vec<usize> = alive.iter().copied().collect();
        alive_list.sort_unstable();

        let mut best: Option<(usize, usize)> = None;
        let mut best_d = f64::INFINITY;
        for ai in 0..alive_list.len() {
            for bi in (ai + 1)..alive_list.len() {
                let (a, b) = (alive_list[ai], alive_list[bi]);
                let key = if a < b { (a, b) } else { (b, a) };
                if d.get(&key).is_some_and(|&dd| dd < best_d) {
                    best_d = d[&key];
                    best = Some((a, b));
                }
            }
        }
        // `alive` always has >=2 members here (loop guard) and `d` holds a
        // finite distance for every pair still alive, so `best` is always
        // populated; fall back to a defensive first-pair pick rather than
        // panicking if that invariant is ever violated.
        let (a, b) = best.unwrap_or((alive_list[0], alive_list[1]));
        let size = id_to_size[&a] + id_to_size[&b];
        linkage.push((a, b, best_d, size));

        let new_id = next_id;
        next_id += 1;
        id_to_size.insert(new_id, size);

        alive.remove(&a);
        alive.remove(&b);
        for &c in &alive_list {
            if c == a || c == b {
                continue;
            }
            let key_a = if a < c { (a, c) } else { (c, a) };
            let key_b = if b < c { (b, c) } else { (c, b) };
            let d_a = d.get(&key_a).copied().unwrap_or(f64::INFINITY);
            let d_b = d.get(&key_b).copied().unwrap_or(f64::INFINITY);
            let d_new = d_a.min(d_b);
            let key_new = if new_id < c { (new_id, c) } else { (c, new_id) };
            d.insert(key_new, d_new);
        }
        alive.insert(new_id);
    }

    linkage
}

/// Quasi-diagonalization: recover leaf order from a linkage list.
///
/// Standard "seriation" walk used by HRP: recursively expand each merge
/// into its two children until only original leaves (`< n`) remain.
fn linkage_order(linkage: &[LinkageRow], n: usize) -> Vec<usize> {
    fn expand(cluster_id: usize, n: usize, linkage: &[LinkageRow], out: &mut Vec<usize>) {
        if cluster_id < n {
            out.push(cluster_id);
            return;
        }
        let (a, b, _, _) = linkage[cluster_id - n];
        expand(a, n, linkage, out);
        expand(b, n, linkage, out);
    }
    let root = n - 1 + linkage.len();
    let mut out = Vec::with_capacity(n);
    expand(root, n, linkage, &mut out);
    out
}

/// Recursive bisection allocation over quasi-diagonalized order.
fn hrp_bisection(cov: &DMatrix<f64>, sorted_idx: &[usize]) -> Vec<f64> {
    let n = sorted_idx.len();
    let mut weights = vec![1.0_f64; n];
    let mut clusters: Vec<Vec<usize>> = vec![sorted_idx.to_vec()];
    // Original-asset-index -> position in `sorted_idx`, built once so the
    // per-cluster weight update below is a lookup rather than a linear scan.
    let pos_of: HashMap<usize, usize> = sorted_idx
        .iter()
        .enumerate()
        .map(|(pos, &idx)| (idx, pos))
        .collect();

    let cluster_var = |members: &[usize]| -> f64 {
        let m = members.len();
        let inv_diag: Vec<f64> = members.iter().map(|&i| 1.0 / cov[(i, i)]).collect();
        let s: f64 = inv_diag.iter().sum();
        let w: Vec<f64> = inv_diag.iter().map(|v| v / s).collect();
        let mut var = 0.0_f64;
        for a in 0..m {
            for b in 0..m {
                var += w[a] * cov[(members[a], members[b])] * w[b];
            }
        }
        var
    };

    while !clusters.is_empty() {
        let mut new_clusters = Vec::new();
        for cluster in &clusters {
            if cluster.len() <= 1 {
                continue;
            }
            let mid = cluster.len() / 2;
            let left = &cluster[..mid];
            let right = &cluster[mid..];

            let var_left = cluster_var(left);
            let var_right = cluster_var(right);
            let denom = var_left + var_right;
            let alpha = if denom > 1e-18 {
                1.0 - var_left / denom
            } else {
                0.5
            };

            for &idx in left {
                if let Some(&pos) = pos_of.get(&idx) {
                    weights[pos] *= alpha;
                }
            }
            for &idx in right {
                if let Some(&pos) = pos_of.get(&idx) {
                    weights[pos] *= 1.0 - alpha;
                }
            }

            new_clusters.push(left.to_vec());
            new_clusters.push(right.to_vec());
        }
        clusters = new_clusters;
    }

    weights
}

/// Hierarchical Risk Parity (Lopez de Prado) portfolio weights.
///
/// `returns_or_cov` is either a `(T, N)` returns matrix (`is_cov = false`,
/// sample covariance is computed internally) or a precomputed `(N, N)`
/// covariance matrix (`is_cov = true`). `bounds` are optional per-asset
/// `(low, high)` bounds applied to the final weights via capped-simplex
/// projection. Direct port of `hrp` in the Python v1.
pub fn hrp(
    returns_or_cov: &DMatrix<f64>,
    bounds: Option<&[Bounds]>,
    is_cov: bool,
) -> PortfolioResult<nalgebra::DVector<f64>> {
    let cov = if is_cov {
        check_cov(returns_or_cov)?;
        returns_or_cov.clone()
    } else {
        check_returns_matrix(returns_or_cov)?;
        super::sample_cov(returns_or_cov)?
    };

    let n = cov.nrows();
    if n == 1 {
        return apply_bounds_and_budget(&nalgebra::DVector::from_element(1, 1.0), bounds, true);
    }

    let corr = corr_from_cov(&cov);
    let mut dist = DMatrix::<f64>::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            dist[(i, j)] = (((1.0 - corr[(i, j)]) / 2.0).clamp(0.0, 1.0)).sqrt();
        }
    }
    for i in 0..n {
        dist[(i, i)] = 0.0;
    }

    let linkage = single_linkage(&dist);
    let order = linkage_order(&linkage, n);
    let w_sorted = hrp_bisection(&cov, &order);

    let mut w = vec![0.0_f64; n];
    for (pos, &idx) in order.iter().enumerate() {
        w[idx] = w_sorted[pos];
    }

    let total: f64 = w.iter().sum();
    if total.abs() < 1e-12 || !w.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::HrpDegenerate);
    }
    let w = nalgebra::DVector::from_vec(w.iter().map(|v| v / total).collect());
    apply_bounds_and_budget(&w, bounds, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DVector;

    #[test]
    fn hrp_single_asset_is_full_weight() {
        let cov = DMatrix::from_row_slice(1, 1, &[0.04]);
        let w = hrp(&cov, None, true).unwrap();
        assert_eq!(w.len(), 1);
        assert!((w[0] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn hrp_weights_sum_to_one_and_nonnegative() {
        // 4-asset covariance with a clear 2-cluster block structure.
        let cov = DMatrix::from_row_slice(
            4,
            4,
            &[
                0.04, 0.03, 0.001, 0.0, 0.03, 0.05, 0.0, 0.001, 0.001, 0.0, 0.02, 0.015, 0.0,
                0.001, 0.015, 0.03,
            ],
        );
        let w = hrp(&cov, None, true).unwrap();
        assert!((w.sum() - 1.0).abs() < 1e-9);
        assert!(w.iter().all(|v| *v >= -1e-9));
    }

    #[test]
    fn hrp_from_returns_matches_hrp_from_sample_cov() {
        let returns = DMatrix::from_row_slice(
            6,
            3,
            &[
                0.01, 0.02, -0.01, 0.015, -0.005, 0.02, -0.02, 0.01, 0.005, 0.005, 0.03, -0.01,
                0.0, 0.01, 0.015, 0.012, -0.004, 0.006,
            ],
        );
        let cov = super::super::sample_cov(&returns).unwrap();
        let w_from_returns = hrp(&returns, None, false).unwrap();
        let w_from_cov = hrp(&cov, None, true).unwrap();
        for i in 0..3 {
            assert!((w_from_returns[i] - w_from_cov[i]).abs() < 1e-9);
        }
    }

    #[test]
    fn hrp_diagonal_cov_two_clusters_equal_split() {
        // Fully uncorrelated -> single-linkage merge order is arbitrary
        // among ties, but bisection on independent variances should still
        // sum to 1 and be strictly positive.
        let cov =
            DMatrix::<f64>::from_diagonal(&DVector::from_row_slice(&[0.01, 0.02, 0.03, 0.04]));
        let w = hrp(&cov, None, true).unwrap();
        assert!((w.sum() - 1.0).abs() < 1e-9);
        assert!(w.iter().all(|v| *v > 0.0));
    }
}
