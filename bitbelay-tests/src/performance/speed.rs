//! Speed test.

use std::hash::BuildHasher;
use std::hash::Hasher as _;
use std::num::NonZeroUsize;
use std::time::Instant;

use bitbelay_providers::Provider;
use bitbelay_report::section;
use bitbelay_report::section::test::Module;
use bitbelay_report::section::test::module;
use byte_unit::Byte;
use statrs::statistics::Data;
use statrs::statistics::Distribution;
use statrs::statistics::Median;

/// A speed test suite.
#[derive(Debug)]
pub struct Test<'a, H: BuildHasher> {
    /// The build hasher.
    build_hasher: &'a H,

    /// The data currently being hashed.
    data: Vec<u8>,

    /// The desired data size as a human readable representation.
    desired_data_size: Byte,

    /// The desired data size in bytes.
    desired_data_size_in_bytes: usize,

    /// The data provider.
    provider: Box<dyn Provider>,

    /// The speed test results in megabytes per second.
    results: Vec<f64>,

    /// The threshold for speed in megabytes per second for the test to be
    /// considered successful.
    threshold: f64,
}

impl<'a, H: BuildHasher> Test<'a, H> {
    /// Creates a new [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// test.run(NonZeroUsize::try_from(5).unwrap());
    /// assert_eq!(test.results().len(), 5);
    /// ```
    pub fn new(
        build_hasher: &'a H,
        provider: Box<dyn Provider>,
        desired_data_size: Byte,
        threshold: f64,
    ) -> Self {
        let desired_data_size_in_bytes = desired_data_size.as_u64() as usize;

        Self {
            build_hasher,
            data: Vec::with_capacity(desired_data_size_in_bytes),
            desired_data_size,
            desired_data_size_in_bytes,
            results: Vec::new(),
            provider,
            threshold,
        }
    }

    /// Gets the [`BuildHasher`] from the [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// assert_eq!(test.build_hasher().hash_one("42"), hasher.hash_one("42"));
    /// ```
    pub fn build_hasher(&self) -> &H {
        self.build_hasher
    }

    /// Gets current data within the [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// // Starts out empty.
    /// assert_eq!(test.data().len(), 0);
    /// assert_eq!(test.data().capacity(), 15000);
    ///
    /// test.run(NonZeroUsize::try_from(1).unwrap());
    ///
    /// // Filled with an iteration is run.
    /// assert_eq!(test.data().len(), 15000);
    /// assert_eq!(test.data().capacity(), 15000);
    /// ```
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Gets desired data size for a [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// assert_eq!(test.desired_data_size(), Byte::from_u64(15000));
    /// ```
    pub fn desired_data_size(&self) -> Byte {
        self.desired_data_size
    }

    /// Gets the [`Provider`] from the [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// assert_eq!(test.provider().name(), provider.name());
    /// ```
    pub fn provider(&self) -> &dyn Provider {
        self.provider.as_ref()
    }

    /// Gets the test results as megabytes read per sec from the [`Test`] by
    /// reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// test.run(NonZeroUsize::try_from(5).unwrap());
    /// assert_eq!(test.results().len(), 5);
    /// ```
    pub fn results(&self) -> &[f64] {
        self.results.as_ref()
    }

    /// Gets the threshold megabytes read per sec for the [`Test`] to be
    /// considered successful.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// assert_eq!(test.threshold(), 1000.0);
    /// ```
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Rehydrates the inner `self.data` from `self.provider` for at least
    /// `self.desired_data_size_in_bytes` bytes.
    fn rehydrate(&mut self) {
        self.data.clear();

        tracing::info!(
            "Generating {:#} of data (prior to running speed test).",
            self.desired_data_size
        );

        // FIXME: this might be able to be improved for some providers that are smarter
        // about allocations than the ones available at the time of writing by providing
        // `math.ceil(desired_data_size_in_bytes / provider.bytes_per_provide)`.
        while self.data.len() < self.desired_data_size_in_bytes {
            self.data.extend_from_slice(self.provider.provide(1)[0]);
        }

        tracing::info!("Finished generating data.");

        if tracing::enabled!(tracing::Level::TRACE) {
            let provided_data_size = Byte::from_u64(self.data.len() as u64);
            tracing::trace!(
                "Desired data size: {:#}, provided data size: {:#}",
                self.desired_data_size,
                provided_data_size
            );
        }
    }

    /// Runs `iterations` speed tests against random data from `self.provider`
    /// and stores the resulting megabytes per second in `self.results`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::speed::Test;
    /// use byte_unit::Byte;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let mut test = Test::new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     Byte::from_u64(15000),
    ///     1000.0,
    /// );
    ///
    /// test.run(NonZeroUsize::try_from(5).unwrap());
    /// assert_eq!(test.results().len(), 5);
    /// ```
    pub fn run(&mut self, iterations: NonZeroUsize) {
        for i in 1..=iterations.get() {
            self.rehydrate();
            self.results
                .push(precision_timed_hash(self.build_hasher, &self.data, i));
        }
    }
}

impl<'a, H: BuildHasher> crate::r#trait::Test for Test<'a, H> {
    fn title(&self) -> &'static str {
        "Performance"
    }

    fn report_section(&self) -> bitbelay_report::section::Test {
        let data = Data::new(self.results.clone());

        // SAFETY: for the [`Data`] distribution, all of the operations below will
        // unwrap, as they always return [`Some`] (this was confirmed by manually
        // examining the code for each of these functions within [`Data`]).
        let mean = data.mean().unwrap();
        let median = data.median();
        let std_dev = data.std_dev().unwrap();

        let mean_module = Module::new(
            if mean >= self.threshold {
                module::Result::Pass
            } else {
                module::Result::Fail
            },
            "Average Speed",
            Some(format!("{:.2} Mb/sec ± {:.2} Mb/sec", mean, std_dev)),
            None,
        );

        let median_module = Module::new(
            if median >= self.threshold {
                module::Result::Pass
            } else {
                module::Result::Fail
            },
            "Median Speed",
            Some(format!("{:.2} Mb/sec", median)),
            None,
        );

        section::test::Builder::default()
            .title("Speed Test")
            .unwrap()
            .description(
                "Runs a set of speed tests for a hash function, including: \n\n  * Comparison of \
                 the mean speed against a predetermined threshold.\n  * Comparison of the median \
                 speed against a predetermined threshold.",
            )
            .unwrap()
            .push_module(mean_module)
            .push_module(median_module)
            // SAFETY: this is manually crafted to always unwrap.
            .try_build()
            .unwrap()
    }
}

/// Performs a hash of the provided data and returns the number of megabytes per
/// second that was processed. Notably, the timer is started as close as
/// possible to the execution of the hash, and it is stopped immediately after
/// the hash is computed—this ensures that the most accurate time that _can_ be
/// generated _is_ generated.
fn precision_timed_hash<H: BuildHasher>(build_hasher: &H, data: &[u8], iteration: usize) -> f64 {
    let mut hasher = build_hasher.build_hasher();

    let now = Instant::now();
    hasher.write(data);
    // NOTE: `black_box()` is required so that the compiler doesn't optimize away
    // the calculation (e.g., because, if the `TRACE` log level isn't enabled, the
    // result of the hash isn't being used).
    let result = std::hint::black_box(hasher.finish());
    let duration = now.elapsed();

    tracing::trace!("Hashing result: {:#x}", result);

    let megabytes = data.len() as f64 / 1_000_000.0;
    let seconds = duration.as_secs_f64();

    let mb_per_second = megabytes / seconds;

    tracing::trace!(
        "[{}] Duration: {:#.2?}, Throughput: {:#.2} Mb/sec",
        iteration,
        duration,
        mb_per_second,
    );

    mb_per_second
}
