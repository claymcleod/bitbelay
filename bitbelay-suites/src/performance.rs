//! Speed test suite.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_tests::performance::Test;
use bitbelay_tests::performance::speed;
use byte_unit::Byte;

pub mod suite;

/// A speed test suite.
#[derive(Debug)]
pub struct Suite<'a, H: BuildHasher> {
    /// The build hasher.
    build_hasher: &'a H,

    /// The performance tests.
    tests: Vec<Test<'a, H>>,
}

impl<'a, H: BuildHasher> Suite<'a, H> {
    /// Gets the [`BuildHasher`] for this [`Suite`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_suites::performance::suite::Builder;
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
    /// use bitbelay_suites::performance::suite::Builder;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_speed_test(
    ///     provider,
    ///     NonZeroUsize::try_from(10).unwrap(),
    ///     "10 KiB".parse::<Byte>().unwrap(),
    ///     1_000.0,
    /// );
    ///
    /// assert_eq!(suite.tests().len(), 1);
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
    /// use bitbelay_suites::performance::suite::Builder;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_speed_test(
    ///     provider,
    ///     NonZeroUsize::try_from(10).unwrap(),
    ///     "10 KiB".parse::<Byte>().unwrap(),
    ///     1_000.0,
    /// );
    ///
    /// assert_eq!(suite.into_tests().len(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_tests(self) -> Vec<Test<'a, H>> {
        self.tests
    }

    /// Runs a [speed test](speed::Test) within the [`Suite`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_suites::performance::suite::Builder;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let mut suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// suite.run_speed_test(
    ///     provider,
    ///     NonZeroUsize::try_from(10).unwrap(),
    ///     "10 KiB".parse::<Byte>().unwrap(),
    ///     1_000.0,
    /// );
    ///
    /// assert_eq!(suite.tests().len(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn run_speed_test(
        &mut self,
        provider: Box<dyn Provider>,
        iterations: NonZeroUsize,
        desired_data_size: Byte,
        threshold: f64,
    ) -> anyhow::Result<()> {
        let mut test = speed::Test::new(self.build_hasher, provider, desired_data_size, threshold);

        test.run(iterations);
        self.tests.push(Test::Speed(test));

        Ok(())
    }
}

impl<'a, H: BuildHasher> crate::r#trait::Suite for Suite<'a, H> {
    fn title(&self) -> &'static str {
        "Performance"
    }

    fn report(&self) -> bitbelay_report::Report {
        let tests = self
            .tests
            .iter()
            .map(|t| t.report_section())
            .collect::<Vec<_>>();

        // SAFETY: this is manually crafted to always unwrap.
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
