//! Avalanching test suite.
//!
//! # Abbreviations
//!
//! Throughout this test suite, you might find the following abbreviations:
//!
//! * **SAC** or **sac** means the "Strict Avalanche Criterion".

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_report::Report;
use bitbelay_tests::avalanche::Test;
use bitbelay_tests::avalanche::sac;

pub mod suite;

/// An error related to a [`Suite`].
#[derive(Debug)]
pub enum Error {
    /// An error with the SAC test.
    StrictAvalancheCriterion(sac::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StrictAvalancheCriterion(err) => write!(f, "sac error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
type Result<T> = std::result::Result<T, Error>;

/// An avalanche test suite.
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
    /// use bitbelay_suites::avalanche::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::<RandomState, 64>::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build()?;
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
    /// use bitbelay_suites::avalanche::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::<RandomState, 64>::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_strict_avalanche_criterion_test(
    ///     provider,
    ///     NonZeroUsize::try_from(10).unwrap(),
    ///     NonZeroUsize::try_from(5_000).unwrap(),
    ///     0.01,
    /// );
    ///
    /// assert_eq!(suite.tests().len(), 1);
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
    /// use bitbelay_suites::avalanche::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::<RandomState, 64>::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_strict_avalanche_criterion_test(
    ///     provider,
    ///     NonZeroUsize::try_from(10).unwrap(),
    ///     NonZeroUsize::try_from(5_000).unwrap(),
    ///     0.01,
    /// );
    ///
    /// assert_eq!(suite.tests().len(), 1);
    /// assert!(matches!(
    ///     suite
    ///         .into_tests()
    ///         .into_iter()
    ///         .next()
    ///         .unwrap()
    ///         .into_strict_avalanche_criterion_test(),
    ///     Some(_)
    /// ));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_tests(self) -> Vec<Test<'a, H, N>> {
        self.tests
    }

    /// Runs a [Strict Avalanche Criterion test](sac::Test) within the
    /// [`Suite`] for a given [`Provider`] and number of iterations.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_suites::avalanche::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::<RandomState, 64>::default()
    ///     .build_hasher(&hasher)?
    ///     .try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_strict_avalanche_criterion_test(
    ///     provider,
    ///     NonZeroUsize::try_from(10).unwrap(),
    ///     NonZeroUsize::try_from(5_000).unwrap(),
    ///     0.01,
    /// );
    ///
    /// assert_eq!(suite.tests().len(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn run_strict_avalanche_criterion_test(
        &mut self,
        provider: Box<dyn Provider>,
        experiments: NonZeroUsize,
        iterations_per_experiment: NonZeroUsize,
        max_deviance: f64,
    ) -> Result<()> {
        let mut test = sac::Test::try_new(
            self.build_hasher,
            provider,
            iterations_per_experiment,
            max_deviance,
        )
        .map_err(Error::StrictAvalancheCriterion)?;

        for i in 1..=experiments.get() {
            if i % 1_000 == 0 && i != 0 {
                tracing::info!("Executed {} experiments.", i);
            }

            test.run_single_experiment()
                .map_err(Error::StrictAvalancheCriterion)?;
        }

        self.tests.push(Test::StrictAvalancheCriterion(test));

        Ok(())
    }
}

impl<'a, H: BuildHasher, const N: usize> crate::r#trait::Suite for Suite<'a, H, N> {
    fn title(&self) -> &'static str {
        "Avalanching"
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
