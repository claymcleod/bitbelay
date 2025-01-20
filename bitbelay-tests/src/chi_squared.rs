//! Chi-squared tests.

use std::hash::BuildHasher;

use bitbelay_report::section;

use crate::r#trait::Test as _;

pub mod goodness_of_fit;

/// A type of chi-squared test.
#[derive(Debug)]
pub enum Test<'a, H: BuildHasher> {
    /// Goodness of fit test.
    GoodnessOfFit(goodness_of_fit::Test<'a, H>),
}

impl<'a, H: BuildHasher> Test<'a, H> {
    /// Gets a reference to a [`goodness_of_fit::Test`] wrapped in [`Some`] if
    /// the [`Test`] is a [`Test::GoodnessOfFit`]. Else, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::Test;
    /// use bitbelay_tests::chi_squared::goodness_of_fit;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::GoodnessOfFit(goodness_of_fit::Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(256).unwrap(),
    ///     0.05,
    /// ));
    ///
    /// assert!(matches!(test.as_goodness_of_fit_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_goodness_of_fit_test(&self) -> Option<&goodness_of_fit::Test<'a, H>> {
        match self {
            Test::GoodnessOfFit(test) => Some(test),
        }
    }

    /// Consumes the [`Test`] and returns a [`goodness_of_fit::Test`] wrapped in
    /// [`Some`] if the [`Test`] is a [`Test::GoodnessOfFit`]. Else, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::Test;
    /// use bitbelay_tests::chi_squared::goodness_of_fit;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::GoodnessOfFit(goodness_of_fit::Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(256).unwrap(),
    ///     0.05,
    /// ));
    ///
    /// assert!(matches!(test.into_goodness_of_fit_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_goodness_of_fit_test(self) -> Option<goodness_of_fit::Test<'a, H>> {
        match self {
            Test::GoodnessOfFit(test) => Some(test),
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
    /// use bitbelay_tests::chi_squared::Test;
    /// use bitbelay_tests::chi_squared::goodness_of_fit;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::GoodnessOfFit(goodness_of_fit::Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(256).unwrap(),
    ///     0.05,
    /// ));
    ///
    /// let section = test.report_section();
    /// // Do something with `section`.
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn report_section(&self) -> section::Test {
        match self {
            Test::GoodnessOfFit(test) => test.report_section(),
        }
    }
}
