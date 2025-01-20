//! Spearman's rank correlation coefficient.
//!
//! # Overview
//!
//! Computes the Spearman rank correlation coefficient as defined by the
//! following formula.
//!
//! $$
//! \rho = 1 - \frac{6\sum\mathrm{d}_{i}^{2}}{n(n^2 - 1)} \text{ where } d_i
//! = R(a_i) - R(b_i)
//! $$
//!
//! # Sources
//!
//! * [Wikipedia] has a relatively informative page on Spearman's correlation
//!   coefficient.
//!
//! [Wikipedia]: https://en.wikipedia.org/wiki/Spearman%27s_rank_correlation_coefficient

use crate::rank;

/// Computes the Spearman rank correlation coefficient between the provided
/// element slices.
///
/// # Results
///
/// If the slices are not the same length or if they are empty, the result is
/// undefined and, as such, [`None`] is returned. In all other cases, the
/// Spearman rank correlation is returned as an [`f64`] in the range of `-1 <=
/// rho <= 1` and may be interpretted as follows:
///
/// * Results near `1` are monotonic, meaning they have a clear positive
///   relationship between the two slices.
/// * Results near `-1` are anti-monotonic, meaning they have a clear negative
///   relationship between the two slices.
/// * Results near `0` indicate no relationship between the two slices.
///
/// # Examples
///
/// ```
/// use approx::assert_relative_eq;
/// use bitbelay_statistics::correlation::spearman;
///
/// // Monotonic relationship, result is `1`.
/// let rho = spearman::correlation(&[2, 1, 4, 3], &[20, 10, 40, 30]);
/// assert_eq!(rho, Some(1.0));
///
/// // Anti-monotonic relationship, result is `-1`.
/// let rho = spearman::correlation(&[2, 1, 4, 3], &[30, 40, 10, 20]);
/// assert_eq!(rho, Some(-1.0));
///
/// // No relationship, result is nearly `0`.
/// let rho = spearman::correlation(
///     &[24, 63, 32, 80, 52, 50, 16, 59],
///     &[56, 95, 54, 51, 63, 17, 80, 90],
/// )
/// .unwrap();
/// assert_relative_eq!(rho, 0.095, epsilon = 1e-3);
/// ```
pub fn correlation<T: Clone + Ord>(a: &[T], b: &[T]) -> Option<f64> {
    // If the slices are not the same length or empty, the result is undefined.
    // Therefore, we return [`None`].
    if a.is_empty() || a.len() != b.len() {
        return None;
    }

    let a = rank(a);
    let b = rank(b);
    let n = a.len() as f64;

    // Sum of the rank differences squared.
    let differences: f64 = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| (a as f64 - b as f64).powi(2))
        .sum();

    Some(1.0 - (6.0 * differences) / (n * (n.powi(2) - 1.0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_lengths() {
        let a = &[1, 2, 3, 4];
        let b = &[5, 6];
        assert_eq!(correlation(a, b), None);
    }

    #[test]
    fn empty() {
        let a: &[usize] = &[];
        let b: &[usize] = &[];
        assert_eq!(correlation(a, b), None);
    }
}
