//! Pearson's correlation coefficient.
//!
//! # Overview
//!
//! Computes the Pearson correlation coefficient as defined by the following
//! formula.
//!
//! $$
//! \rho = \frac{\sum(a_{i} - \bar{a})(b_{i} -
//! \bar{b})}{\sqrt{\sum(a_{i} - \bar{a})^2}
//! \sqrt{\sum(b_{i} - \bar{b})^2}}
//! $$
//!
//! The above transforms to the following formula, which is easier to write
//! and also what is actually implemented.
//!
//! $$
//! \rho = \frac{n\sum{a_{i}b_{i}} - \sum{a_{i}}
//! \sum{b_{i}}}{\sqrt{n\sum{a_{i}^2-(\sum{a_{i}})^2}}
//! \sqrt{n\sum{b_{i}^2-(\sum{b_{i}})^2}}}
//! $$
//!
//! # Sources
//!
//! * [Wikipedia] has a relatively informative page on Pearson's correlation
//!   coefficient.
//!
//! [Wikipedia]: https://en.wikipedia.org/wiki/Pearson_correlation_coefficient

/// Computes the Pearson correlation coefficient between the provided element
/// slices.
///
/// # Results
///
/// If the slices are not the same length or if they are empty, the result is
/// undefined and, as such, [`None`] is returned. In all other cases, the
/// Pearson correlation is returned as an [`f64`] in the range of `-1 <= rho <=
/// 1` and may be interpretted as follows:
///
/// * Results near `1` have a clear positive, linear relationship between the
///   two slices.
/// * Results near `-1` are anti-monotonic, meaning they have a clear negative,
///   linear relationship between the two slices.
/// * Results near `0` indicate no linear relationship between the two slices.
///
/// # Examples
///
/// ```
/// use approx::assert_relative_eq;
/// use bitbelay_statistics::correlation::pearson;
///
/// // Positive linear relationship, result is `1`.
/// let rho = pearson::correlation(&[1.0, 2.0, 3.0, 4.0], &[5.0, 6.0, 7.0, 8.0]);
/// assert_eq!(rho, Some(1.0));
///
/// // Negative linear relationship, result is `-1`.
/// let rho = pearson::correlation(&[1.0, 2.0, 3.0, 4.0], &[8.0, 7.0, 6.0, 5.0]);
/// assert_eq!(rho, Some(-1.0));
///
/// // No relationship, result is nearly `0`.
/// let rho = pearson::correlation(
///     &[24.0, 63.0, 32.0, 80.0, 52.0, 50.0, 16.0, 59.0],
///     &[56.0, 95.0, 54.0, 51.0, 63.0, 17.0, 80.0, 90.0],
/// )
/// .unwrap();
/// assert_relative_eq!(rho, 0.018, epsilon = 1e-3);
/// ```
pub fn correlation(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }

    let sum_a: f64 = a.iter().sum();
    let sum_b: f64 = b.iter().sum();

    let sum_a_squared: f64 = a.iter().map(|a| a.powi(2)).sum();
    let sum_b_squared: f64 = b.iter().map(|b| b.powi(2)).sum();

    let sum_a_times_b: f64 = a.iter().zip(b.iter()).map(|(a, b)| a * b).sum();

    let n: f64 = a.len() as f64;

    let num = n * sum_a_times_b - sum_a * sum_b;
    let denom = ((n * sum_a_squared - sum_a.powi(2)) * (n * sum_b_squared - sum_b.powi(2))).sqrt();

    if denom == 0.0 {
        return None;
    }

    Some(num / denom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_zero_elements() {
        let elements = &[0.0, 0.0, 0.0];
        assert_eq!(correlation(elements, elements), None);
    }

    #[test]
    fn different_lengths() {
        let a = &[1.0, 2.0, 3.0, 4.0];
        let b = &[5.0, 6.0];
        assert_eq!(correlation(a, b), None);
    }

    #[test]
    fn empty() {
        let a = &[];
        let b = &[];
        assert_eq!(correlation(a, b), None);
    }
}
