//! Avalanche tests.

use std::hash::BuildHasher;

use bitbelay_report::section;

use crate::r#trait::Test as _;

pub mod sac;

/// A type of avalanche test.
#[derive(Debug)]
pub enum Test<'a, H: BuildHasher, const N: usize> {
    /// Strict Avalanche Criterion test.
    StrictAvalancheCriterion(sac::Test<'a, H, N>),
}

impl<'a, H: BuildHasher, const N: usize> Test<'a, H, N> {
    /// Gets a reference to a [`sac::Test`] wrapped in [`Some`] if
    /// the [`Test`] is a [`Test::StrictAvalancheCriterion`]. Else, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac;
    /// use bitbelay_tests::avalanche::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::StrictAvalancheCriterion(
    ///     sac::Test::<RandomState, 64>::try_new(
    ///         &hasher,
    ///         Box::new(AlphanumericProvider::new(10)),
    ///         NonZeroUsize::try_from(1000).unwrap(),
    ///         0.01,
    ///     )
    ///     .unwrap(),
    /// );
    ///
    /// assert!(matches!(test.as_strict_avalanche_criterion_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_strict_avalanche_criterion_test(&self) -> Option<&sac::Test<'a, H, N>> {
        match self {
            Test::StrictAvalancheCriterion(test) => Some(test),
        }
    }

    /// Consumes the [`Test`] and returns a [`sac::Test`] wrapped in [`Some`] if
    /// the [`Test`] is a [`Test::StrictAvalancheCriterion`]. Else, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac;
    /// use bitbelay_tests::avalanche::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::StrictAvalancheCriterion(
    ///     sac::Test::<RandomState, 64>::try_new(
    ///         &hasher,
    ///         Box::new(AlphanumericProvider::new(10)),
    ///         NonZeroUsize::try_from(1000).unwrap(),
    ///         0.01,
    ///     )
    ///     .unwrap(),
    /// );
    ///
    /// assert!(matches!(
    ///     test.into_strict_avalanche_criterion_test(),
    ///     Some(_)
    /// ));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_strict_avalanche_criterion_test(self) -> Option<sac::Test<'a, H, N>> {
        match self {
            Test::StrictAvalancheCriterion(test) => Some(test),
        }
    }

    /// Generates a report section for the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac;
    /// use bitbelay_tests::avalanche::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::StrictAvalancheCriterion(
    ///     sac::Test::<RandomState, 64>::try_new(
    ///         &hasher,
    ///         Box::new(AlphanumericProvider::new(10)),
    ///         NonZeroUsize::try_from(1000).unwrap(),
    ///         0.01,
    ///     )
    ///     .unwrap(),
    /// );
    ///
    /// let results = test.report_section();
    /// // Include the section in a report.
    /// ```
    pub fn report_section(&self) -> section::Test {
        match self {
            Test::StrictAvalancheCriterion(test) => test.report_section(),
        }
    }
}
