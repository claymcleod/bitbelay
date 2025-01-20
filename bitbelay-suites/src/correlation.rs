//! Chi-squared test suite.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_report::Report;
use bitbelay_tests::correlation::Test;
use bitbelay_tests::correlation::bitwise;

pub mod suite;

/// A chi-squared test suite.
#[derive(Debug)]
pub struct Suite<'a, H: BuildHasher, const N: usize> {
    /// The hash function builder.
    build_hasher: &'a H,

    /// The tests that have been run within this suite.
    tests: Vec<Test<'a, H, N>>,
}

impl<'a, H: BuildHasher, const N: usize> Suite<'a, H, N> {
    /// Gets the [`BuildHasher`] for this [`Suite`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_suites::correlation::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build::<64>()?;
    ///
    /// // Used as a surrogate to test that the [`BuildHasher`]s are the same.
    /// assert_eq!(suite.build_hasher().hash_one("42"), hasher.hash_one("42"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn build_hasher(&self) -> &H {
        self.build_hasher
    }

    /// Gets the [`Test`]s run within this [`Suite`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_suites::correlation::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build::<64>()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_bitwise_test(provider, NonZeroUsize::try_from(10).unwrap(), 0.05);
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert_eq!(
    ///     suite
    ///         .tests()
    ///         .first()
    ///         .unwrap()
    ///         .as_bitwise_test()
    ///         .unwrap()
    ///         .bit_values()[0]
    ///         .len(),
    ///     10
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn tests(&self) -> &[Test<'a, H, N>] {
        self.tests.as_ref()
    }

    /// Consumes `self` and returns the [`Test`]s run within this [`Suite`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_suites::correlation::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build::<64>()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_bitwise_test(provider, NonZeroUsize::try_from(10).unwrap(), 0.05);
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert!(matches!(
    ///     suite
    ///         .into_tests()
    ///         .into_iter()
    ///         .next()
    ///         .unwrap()
    ///         .into_bitwise_test(),
    ///     Some(_)
    /// ));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_tests(self) -> Vec<Test<'a, H, N>> {
        self.tests
    }

    /// Runs a [bitwise test](bitwise::Test) within the [`Suite`] for a given
    /// [`Provider`] and number of iterations.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_suites::correlation::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build::<64>()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_bitwise_test(provider, NonZeroUsize::try_from(10).unwrap(), 0.05);
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert_eq!(
    ///     suite
    ///         .tests()
    ///         .first()
    ///         .unwrap()
    ///         .as_bitwise_test()
    ///         .unwrap()
    ///         .bit_values()[0]
    ///         .len(),
    ///     10
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn run_bitwise_test(
        &mut self,
        mut provider: Box<dyn Provider>,
        iterations: NonZeroUsize,
        threshold: f64,
    ) -> anyhow::Result<()> {
        let mut test = bitwise::Test::new(self.build_hasher, threshold);
        test.run(&mut provider, iterations);
        self.tests.push(Test::Bitwise(test));

        Ok(())
    }
}

impl<'a, H: BuildHasher, const N: usize> crate::r#trait::Suite for Suite<'a, H, N> {
    fn title(&self) -> &'static str {
        "Chi Squared"
    }

    fn report(&self) -> Report {
        let tests = self
            .tests
            .iter()
            .map(|t| t.report_section())
            .collect::<Vec<_>>();

        let mut builder = bitbelay_report::Builder::default()
            .title(self.title())
            .unwrap();

        for test in tests {
            builder = builder.push_test_result(test);
        }

        // SAFETY: this is manually crafted to always unwrap.
        builder.try_build().unwrap()
    }
}
