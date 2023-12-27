//! Chi-squared test suite.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_report::Report;
use bitbelay_tests::chi_squared::goodness_of_fit;
use bitbelay_tests::chi_squared::Test;

pub mod suite;

/// A chi-squared test suite.
#[derive(Debug)]
pub struct Suite<'a, H: BuildHasher> {
    /// The hash function builder.
    build_hasher: &'a H,

    /// The tests that have been run within this suite.
    tests: Vec<Test<'a, H>>,

    /// The number of buckets to use within each test.
    buckets: NonZeroUsize,
}

impl<'a, H: BuildHasher> Suite<'a, H> {
    /// Gets the number of buckets for the tests run within this [`Suite`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// assert_eq!(suite.buckets().get(), 256);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn buckets(&self) -> NonZeroUsize {
        self.buckets
    }

    /// Gets the [`BuildHasher`] for this [`Suite`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::default().build_hasher(&hasher)?.try_build()?;
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
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_goodness_of_fit(provider, NonZeroUsize::try_from(10).unwrap(), 0.05);
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert_eq!(
    ///     suite
    ///         .tests()
    ///         .first()
    ///         .unwrap()
    ///         .as_goodness_of_fit_test()
    ///         .unwrap()
    ///         .buckets()
    ///         .iter()
    ///         .sum::<usize>(),
    ///     10
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn tests(&self) -> &[Test<'a, H>] {
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
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_goodness_of_fit(provider, NonZeroUsize::try_from(10).unwrap(), 0.05);
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert!(matches!(
    ///     suite
    ///         .into_tests()
    ///         .into_iter()
    ///         .next()
    ///         .unwrap()
    ///         .into_goodness_of_fit_test(),
    ///     Some(_)
    /// ));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_tests(self) -> Vec<Test<'a, H>> {
        self.tests
    }

    /// Runs a [goodness of fit test](goodness_of_fit::Test) within the
    /// [`Suite`] for a given [`Provider`] and number of iterations.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_goodness_of_fit(provider, NonZeroUsize::try_from(10).unwrap(), 0.05);
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert_eq!(
    ///     suite
    ///         .tests()
    ///         .first()
    ///         .unwrap()
    ///         .as_goodness_of_fit_test()
    ///         .unwrap()
    ///         .buckets()
    ///         .iter()
    ///         .sum::<usize>(),
    ///     10
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn run_goodness_of_fit(
        &mut self,
        provider: Box<dyn Provider>,
        iterations: NonZeroUsize,
        threshold: f64,
    ) {
        let mut test =
            goodness_of_fit::Test::new(self.build_hasher, provider, self.buckets, threshold);

        for i in 0..iterations.get() {
            if i % 1_000 == 0 && i != 0 {
                tracing::info!("Executed {} iterations.", i);
            }

            test.single_iteration();
        }

        self.tests.push(Test::GoodnessOfFit(test));
    }
}

impl<'a, H: BuildHasher> crate::r#trait::Suite for Suite<'a, H> {
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
