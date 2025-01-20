//! Goodness of fit test.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_report::section;
use bitbelay_report::section::test;
use bitbelay_report::section::test::Module;
use bitbelay_report::section::test::module;
use bitbelay_statistics::chi_squared::UniformPearsonTest;
use colored::Colorize;

/// A chi-squared goodness of fit test.
#[derive(Debug)]
pub struct Test<'a, H: BuildHasher> {
    /// The hash function builder.
    build_hasher: &'a H,

    /// The data provider.
    provider: Box<dyn Provider>,

    /// The number of buckets to use within the test.
    buckets: Vec<usize>,

    /// The threshold of statistical signficance to use.
    threshold: f64,
}

impl<'a, H: BuildHasher> Test<'a, H> {
    /// Creates a new [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let test = Test::new(
    ///     &hasher,
    ///     provider.clone(),
    ///     NonZeroUsize::try_from(2048).unwrap(),
    ///     0.05,
    /// );
    ///
    /// assert_eq!(test.build_hasher().hash_one("42"), hasher.hash_one("42"));
    /// assert_eq!(test.provider().name(), provider.name());
    /// assert_eq!(test.buckets().len(), 2048);
    /// ```
    pub fn new(
        build_hasher: &'a H,
        provider: Box<dyn Provider>,
        num_buckets: NonZeroUsize,
        threshold: f64,
    ) -> Self {
        Test {
            build_hasher,
            provider,
            buckets: vec![0; num_buckets.get()],
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
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let test = Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(2048).unwrap(),
    ///     0.05,
    /// );
    ///
    /// assert_eq!(test.build_hasher().hash_one("42"), hasher.hash_one("42"));
    /// ```
    pub fn build_hasher(&self) -> &H {
        self.build_hasher
    }

    /// Gets the [`Provider`] from the [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let test = Test::new(
    ///     &hasher,
    ///     provider.clone(),
    ///     NonZeroUsize::try_from(2048).unwrap(),
    ///     0.05,
    /// );
    ///
    /// assert_eq!(test.provider().name(), provider.name());
    /// ```
    pub fn provider(&self) -> &dyn Provider {
        self.provider.as_ref()
    }

    /// Gets the buckets from the [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let hasher = RandomState::new();
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let test = Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(2048).unwrap(),
    ///     0.05,
    /// );
    ///
    /// assert_eq!(test.buckets().len(), 2048);
    /// ```
    pub fn buckets(&self) -> &Vec<usize> {
        &self.buckets
    }

    /// Gets the threshold from the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let test = Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(2048).unwrap(),
    ///     0.05,
    /// );
    ///
    /// assert_eq!(test.threshold(), 0.05);
    /// ```
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Performs a single iteration of the test.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let mut test = Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(2048).unwrap(),
    ///     0.05,
    /// );
    ///
    /// test.single_iteration();
    ///
    /// assert_eq!(test.buckets().iter().sum::<usize>(), 1);
    /// ```
    pub fn single_iteration(&mut self) {
        let data = *self.provider.provide(1).first().unwrap();
        let hash = self.build_hasher.hash_one(data);
        let bucket = (hash as usize) % self.buckets.len();

        self.buckets[bucket] += 1;
    }

    /// Gets the p-value of the test.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::chi_squared::goodness_of_fit::Test;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let mut test = Test::new(
    ///     &hasher,
    ///     provider,
    ///     NonZeroUsize::try_from(256).unwrap(),
    ///     0.05,
    /// );
    ///
    /// for _ in 0..10 {
    ///     test.single_iteration();
    /// }
    ///
    /// // Not enough data to compute a p-value.
    /// assert!(test.p_value().is_none());
    ///
    /// for _ in 0..2048 {
    ///     test.single_iteration();
    /// }
    ///
    /// assert!(test.p_value().unwrap() <= 1.0);
    /// ```
    pub fn p_value(&self) -> Option<f64> {
        UniformPearsonTest::goodness_of_fit(self.buckets())
    }
}

impl<'a, H: BuildHasher> crate::r#trait::Test for Test<'a, H> {
    fn title(&self) -> &'static str {
        "Goodness of Fit"
    }

    fn report_section(&self) -> bitbelay_report::section::Test {
        let (result, value, details) = if let Some(p_value) = self.p_value() {
            if p_value > self.threshold {
                (
                    module::Result::Pass,
                    Some(format!("{:.2}", p_value)),
                    Some(format!(
                        "The resulting p-value of {:.2} was greater than (and, thus, failed to \
                         reach) the predetermined threshold of statistical significance set at \
                         {:.2}. As such, the null hypothesis that the observed data follows a \
                         random, uniform distribution **cannot** be rejected. In other words, \
                         this indicates that the differences between the observed frequencies and \
                         the expected frequencies under a random, uniform distribution are \
                         **not** statistically significant.",
                        p_value, self.threshold
                    )),
                )
            } else {
                (
                    module::Result::Fail,
                    Some(format!("{:.2}", p_value)),
                    Some(format!(
                        "The resulting p-value of {:.2} was less than (and, thus, reached) the \
                         predetermined threshold of statistical significance set at {:.2}. As \
                         such, the null hypothesis that the observed data follows a random, \
                         uniform distribution **is** rejected. In other words, this indicates \
                         that the differences between the observed frequencies and the expected \
                         frequencies under a random, uniform distribution **are** statistically \
                         significant.",
                        p_value, self.threshold
                    )),
                )
            }
        } else {
            (
                module::Result::Inconclusive,
                None,
                Some(String::from("The p-value was not able to be computed.")),
            )
        };

        let iterations = self.buckets().iter().sum::<usize>();

        // SAFETY: all of the pieces of this [`Builder`] are hand-crafted to not
        // fail, so all of the below will unwrap.
        get_report_base(self.provider.as_ref(), iterations)
            .push_module(Module::new(
                result,
                "Failure to Reject the Null Hypothesis",
                value,
                details,
            ))
            .try_build()
            .unwrap()
    }
}

/// Populates the boilerplate report information within a
/// [`Test`](section::Test).
pub fn get_report_base(provider: &dyn Provider, iterations: usize) -> section::test::Builder {
    let overview =
        "The chi-squared goodness of fit test assesses whether there is a significant difference \
         between an observed distribution of data and a chosen theoretical distribution.\n\nThe \
         test works by computing the chi-squared statistic, which quantifies the extent of \
         divergence between the observed frequencies and the expected frequencies for a selected \
         theoretical distribution. The computed chi-squared statistic is then used to calculate a \
         p-value, which represents the probability of observing a statistic as extreme as, or \
         more extreme than, the one calculated if the null hypothesis were true. In this context, \
         the null hypothesis posits that there is no significant difference between the observed \
         and theoretical distributions; if the p-value is below a predetermined threshold, the \
         null hypothesis is rejected, indicating that the differences between the observed \
         frequencies and the expected frequencies are statistically signficant.";

    let relation =
        "Many hash-based data structures work by computing the hash of an input value and binning \
         the resulting hashed value to a finite set of buckets (usually via a modulo operation). \
         One desirable characteristic of a hash function is its ability to uniformly distribute \
         hashed values across these buckets (i.e., each bucket gets approximately the same number \
         of hashed values). This uniformity is important, as it ensures balanced load \
         distribution and enhances the efficiency of data insertion and retrieval operations in \
         data structures such as hash tables.\n\nIdeally, the mapping of an input value to its \
         assigned bucket would be similar to assigning it to a random bucket from a uniform \
         distribution (i.e., every bucket is equally likely to be assigned). To evaluate how \
         effective a hash function is at evenly distributing hashed values amongst a set of \
         buckets, we can apply the chi-squared goodness of fit test comparing (a) the frequency \
         of observed hashed values assigned to a set of buckets against (b) the expected \
         frequency if the buckets were assigned from a random, uniform distribution.";

    let algorithm =
        "For a specified hash function, data provider, and predefined number of buckets:\n\n(1) \
         An array with a length matching the number of buckets is allocated. This represents the \
         number of hashes that are assigned to each respective bucket. Each value in the array is \
         initialized to 0 to indicate that no values have been assigned to any bucket yet.\n\n(2) \
         The following process is carried out for a specified number of iterations:\n\n  * A \
         random input is generated using the specified data provider.\n  * The input data is \
         hashed and assigned to one of the buckets:\n    * The hash is divided by the number of \
         buckets.\n    * The remainder of that operation is used as the bucket index.\n  * The \
         value for the selected bucket index is incremented by 1.\n\nIn this way, the array now \
         contains the frequency of hash values assigned to each bucket.\n\n(3) The expected \
         frequency of values for each bucket under a normal distribution is simply the total \
         number of iterations divided by the number of buckets.\n\n(4) The chi-squared statistic \
         can now be calculated, and a p-value can be backed out by calculating the cumulative \
         distribution function (CDF) for the chi-squared statistic. The CDF must be calculated \
         for the chi-squared _distribution_ given the appropriate degrees of freedom for a \
         goodness of fit test (in this case, `number of buckets - 1`).";

    let interpretation =
        "Under this test design:\n\n* A p-value that is greater than or equal to the \
         pre-determined signficance value (typically, 0.05) is **good**, as it means there _is \
         not_ enough evidence to reject the null hypothesis (and, under this test, suggests there \
         is no significant difference between the observed distribution of hashed values and a \
         theoretical uniform distribution).\n\n* A p-value that is less than the pre-determined \
         signficance value is **bad**, as it means there _is_ enough evidence to reject the null \
         hypothesis (and, under this test, suggests there is a significant difference between the \
         observed distribution of hashed values and a theoretical uniform distribution).";

    let sources = "* https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test#Chi-squared_goodness_of_fit_test";

    test::Builder::default()
        .title(format!(
            "Goodness of Fit / {} / {} iterations",
            provider.name(),
            iterations
        ))
        .unwrap()
        .description(format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            "Overview".italic(),
            overview,
            "Relation to Hashing".italic(),
            relation,
            "Algorithm".italic(),
            algorithm,
            "Interpretation".italic(),
            interpretation,
            "Sources".italic(),
            sources
        ))
        .unwrap()
}
