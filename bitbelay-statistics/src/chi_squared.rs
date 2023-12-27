//! Chi-squared statistical tests.
//!
//! # Supported Tests
//!
//! At present, only the following Chi-squared tests are supported:
//!
//! * A [Pearson goodness of fit](pearson-chi-squared-test) against a
//!   theoretrical random, uniform distribution via
//!   [`UniformPearsonTest::goodness_of_fit()`].
//!
//! Other tests may be added in the future as needed.
//!
//! # Sources
//!
//! To learn more, please see these Wikipedia pages:
//!
//! * The main page for the chi-squared distribution
//!   ([link][chi-squared-distribution]).
//! * A general overview of Pearson's chi-squared test
//!   ([link][pearson-chi-squared-test]).
//! * An intuitive description of how the p-value is calculated and then
//!   interpretted ([link][p-value-explanation]).
//! * A worked example ([link][example]).
//!
//! [chi-squared-distribution]: https://en.wikipedia.org/wiki/Chi-squared_distribution
//! [pearson-chi-squared-test]:
//!     https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test
//! [p-value-explanation]: https://en.wikipedia.org/wiki/Chi-squared_distribution#Computational_methods
//! [example]:
//!     https://en.wikipedia.org/wiki/Pearson's_chi-squared_test#Chi-squared_goodness_of_fit_test

use std::f64::NAN;

use statrs::distribution::ChiSquared;
use statrs::distribution::ContinuousCDF as _;

/// Generates the chi-squared (X^2) test statistic for a given observed
/// distribution against a theoretical, uniformly distributed distribution.
///
/// # Notes
///
/// * If the number of expected items per bucket under the random, uniform
///   distribution is not at least 5, then no result is returned.
/// * To simplify the code, the theoretical distribution is hardcoded to model a
///   uniform distribution. This means that the expected values are determined
///   by taking the total number of observations and spreading them equally
///   amongst the possible slots.
/// * As such, this function cannot be used to calculate chi-squared for
///   arbitrary multinomial distributions (though it may be extended to do so in
///   the future).
pub(crate) fn chi_squared_uniform(observations: &[usize]) -> Option<f64> {
    let expected = observations.iter().sum::<usize>() as f64 / observations.len() as f64;

    if expected < 5.0 {
        return None;
    }

    let chi_squared = observations.iter().fold(0.0, |acc, &count| {
        let difference = count as isize - expected as isize;
        acc + (difference.pow(2) as f64) / expected
    });

    Some(chi_squared)
}

/// Pearson chi-squared tests for a theoretical uniform distribution.
///
/// # Notes
///
/// * To simplify the code, all tests assume a theoretical uniform distrbution
///   rather than arbitrary multinomial distributions (though the code may be
///   extended to support that in the future).
/// * Though there are Pearson tests for goodness of fit, homogeneity, and
///   independence, only goodness of fit is implemented at present. This may
///   change in the future if other tests are needed.
///
/// # Sources
///
/// * [Wikipedia] explains the different types of Pearson chi-squared tests.
///
/// [Wikipedia]: https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test
#[allow(missing_debug_implementations)]
pub struct UniformPearsonTest;

impl UniformPearsonTest {
    /// Performs a goodness of fit test for an observed distribution against a
    /// theoretical uniform distribution using the chi-squared statistic.
    ///
    /// To determine whether the null hypothesis can be rejected, you should
    /// compare the p-value returned by this function to a chosen
    /// significance value (generally, `0.05`, but referred to as `p`
    /// below):
    ///
    /// * If the returned value is less than `p`, then the results are
    ///   statistically significant and the null hypothesis **can be rejected**
    ///   (indiciating that one cannot conclude the observed data arose from a
    ///   random, uniform distribution).
    /// * If the returned value is greater than or equal to `p`, then the null
    ///   hypothesis cannot be rejected.
    ///
    /// To make it simpler, in our case, where we wish to test whether hash
    /// values are randomly distributed according to a uniform distribution
    /// over the bit-space (and assuming a significance level of `0.05`):
    ///
    /// * A value of less than `0.05` is **bad** because you _cannot_ conclude
    ///   that the observed values were sampled from a random, uniform
    ///   distribution.
    /// * A value of greater than or equal to `0.05` is **good**, because you
    ///   _can_ conclude, under the test, that the observed values were sampled
    ///   from a random, uniform distribution.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_statistics::chi_squared::UniformPearsonTest;
    ///
    /// // A set of observations that are perfectly uniform.
    /// let observations: &[usize] = &[10, 10, 10, 10, 10];
    /// let p = UniformPearsonTest::goodness_of_fit(&observations).unwrap();
    ///
    /// // The null hypothesis _cannot_ be rejected under the test, and we can conclude
    /// // that the data was sampled from a uniform, random distribution.
    /// assert!(p >= 0.05);
    ///
    /// // A set of observations that are _not_ uniform.
    /// let observations: &[usize] = &[500, 10, 10, 10, 10];
    /// let p = UniformPearsonTest::goodness_of_fit(&observations).unwrap();
    ///
    /// // The null hypothesis _cannot_ be rejected under the test, and we can conclude
    /// // that the data was sampled from a uniform, random distribution.
    /// assert!(p < 0.05);
    /// ```
    ///
    /// # Notes
    ///
    /// * The calculated chi-squared statistic is compared against the
    ///   continuous distribution function of a chi-squared distribution with
    ///   `observations.len() - 1` degrees of freedom (this is appropriate for a
    ///   Pearson goodness of fit test).
    /// * Calculating the CDF value for the observed chi-squared statistic
    ///   describes the probability that an observed statistic is _less_ extreme
    ///   than the observed value. In this case, we're interested in the
    ///   p-value, which is the probability that an observed statistic is _more_
    ///   extreme than the observed value. As such, the final return (p-)value
    ///   is `1.0 - cdf_percentile`.
    pub fn goodness_of_fit(observations: &[usize]) -> Option<f64> {
        let chi_squared_statistic = chi_squared_uniform(observations)?;
        let degrees_of_freedom = observations.len() as f64 - 1.0;

        let percentile = ChiSquared::new(degrees_of_freedom)
            .unwrap_or_else(|_| {
                // SAFETY: this would be highly irregular to fail with the inputs that
                // are supported. As such, any failure to instantiate this should panic
                // and be investigated further.
                panic!(
                    "could not create chi-squared distribution with {} degrees of freedom",
                    degrees_of_freedom
                )
            })
            .cdf(chi_squared_statistic);

        if percentile == NAN {
            return None;
        }

        Some(1.0 - percentile)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    const TOLERANCE: f64 = 1e-3;

    #[test]
    fn test_simple_goodness_of_fit() {
        let observations = vec![50, 60, 40, 47, 53];

        // Ensures the chi-squared calculates correctly.
        let css = chi_squared_uniform(&observations);
        assert_relative_eq!(css.unwrap(), 4.36, epsilon = TOLERANCE);

        // Ensures the uniform pearson
        let p = UniformPearsonTest::goodness_of_fit(&observations);
        assert_relative_eq!(p.unwrap(), 0.359472, epsilon = TOLERANCE);
    }

    #[test]
    fn test_zero_chi_squared_uniform() {
        let observations = vec![5, 5, 5, 5, 5];

        // Ensures the chi-squared calculates correctly.
        let css = chi_squared_uniform(&observations);
        assert_relative_eq!(css.unwrap(), 0.0, epsilon = TOLERANCE);

        // Ensures the uniform pearson
        let p = UniformPearsonTest::goodness_of_fit(&observations);
        assert_relative_eq!(p.unwrap(), 1.0, epsilon = TOLERANCE);
    }

    #[test]
    fn test_biases_chi_squared_uniform() {
        let observations = vec![1000, 0, 0, 0];

        // Ensures the chi-squared calculates correctly.
        let css = chi_squared_uniform(&observations);
        assert_relative_eq!(css.unwrap(), 3000.0, epsilon = TOLERANCE);

        // Ensures the uniform pearson
        let p = UniformPearsonTest::goodness_of_fit(&observations);
        assert_relative_eq!(p.unwrap(), 0.0, epsilon = TOLERANCE);
    }
}
