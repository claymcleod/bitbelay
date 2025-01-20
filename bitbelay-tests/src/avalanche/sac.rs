//! Strict avalanche criterion test.
//!
//! # Sources
//!
//! * [Wikipedia] has a fairly good explanation of the SAC test.
//!
//! [Wikipedia]: https://en.wikipedia.org/wiki/Avalanche_effect#Strict_avalanche_criterion

pub mod experiment;

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_report::section;
use bitbelay_report::section::test::Builder;
use bitbelay_report::section::test::Module;
use bitbelay_report::section::test::module;
use colored::Colorize;
pub use experiment::Experiment;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;

lazy_static! {
    static ref ONE_PCT_CHAR: String = ".".green().to_string();
    static ref FIVE_PCT_CHAR: String = "?".yellow().to_string();
    static ref OTHER_PCT_CHAR: String = "!".red().to_string();
}

/// An error related to a [`Test`].
#[derive(Debug)]
pub enum Error {
    /// An experiment error.
    Experiment(experiment::Error),

    /// An invalid value was passed for max deviance.
    InvalidMaxDeviance(f64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Experiment(err) => write!(f, "experiment error: {err}"),
            Error::InvalidMaxDeviance(value) => {
                write!(
                    f,
                    "max deviance must be between 0.0 and 1.0, received {value}"
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
type Result<T> = std::result::Result<T, Error>;

/// The results of a [`Test`](section::Test).
#[derive(Debug)]
pub struct Results {
    /// Whether the test succeeded or not.
    pub succeeded: bool,

    /// The maximum bias we encountered.
    ///
    /// * The first item in the tuple is the index where the max bias occurred.
    /// * The second item in the tuple is the bias itself.
    pub max_bias: (usize, OrderedFloat<f64>),

    /// The offset of each bit in the output from the expected bit flip
    /// probability.
    pub bit_bias_offsets: Vec<(usize, OrderedFloat<f64>)>,
}

/// A strict avalanche criterion test.
#[derive(Debug)]
pub struct Test<'a, H: BuildHasher, const N: usize> {
    /// The build hasher.
    build_hasher: &'a H,

    /// The data provider.
    provider: Box<dyn Provider>,

    /// The total number of bit flips for each bit in the output hash.
    bit_flips: [usize; N],

    /// The number of iterations within each experiment.
    iterations_per_experiment: NonZeroUsize,

    /// The total number of experiments that have been carried out.
    total_experiments: usize,

    /// The maximum deviance that any single bit can have from `0.5` for the
    /// test to be considered successful.
    ///
    /// Note that this is a fraction (`0.01`), not a percentage (`1`).
    max_deviance: f64,
}

impl<'a, H: BuildHasher, const N: usize> Test<'a, H, N> {
    /// Creates a new [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(test.bit_flips().iter().sum::<usize>(), 0);
    /// assert_eq!(test.total_experiments(), 0);
    /// ```
    pub fn try_new(
        build_hasher: &'a H,
        provider: Box<dyn Provider>,
        iterations_per_experiment: NonZeroUsize,
        max_deviance: f64,
    ) -> Result<Self> {
        if !(0.0..=1.0).contains(&max_deviance) {
            return Err(Error::InvalidMaxDeviance(max_deviance));
        }

        Ok(Self {
            build_hasher,
            provider,
            bit_flips: [0usize; N],
            iterations_per_experiment,
            total_experiments: 0,
            max_deviance,
        })
    }

    /// Gets the build hasher for this [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// // Used as a surrogate to test that the [`BuildHasher`]s are the same.
    /// assert_eq!(test.build_hasher().hash_one("42"), hasher.hash_one("42"));
    /// ```
    pub fn build_hasher(&self) -> &H {
        self.build_hasher
    }

    /// Gets the data provider for this [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     provider.clone(),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(test.provider().name(), provider.name());
    /// ```
    pub fn provider(&self) -> &dyn Provider {
        self.provider.as_ref()
    }

    /// Gets the current number of flips for each output bit in the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(test.bit_flips().iter().sum::<usize>(), 0);
    /// ```
    pub fn bit_flips(&self) -> [usize; N] {
        self.bit_flips
    }

    /// Gets the number of iterations for each experiment in the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(test.iterations_per_experiment().get(), 1000);
    /// ```
    pub fn iterations_per_experiment(&self) -> NonZeroUsize {
        self.iterations_per_experiment
    }

    /// Gets the number of experiments that have been run within the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(test.total_experiments(), 0);
    /// ```
    pub fn total_experiments(&self) -> usize {
        self.total_experiments
    }

    /// Gets the max deviance allowed for any bit within the [`Test`] for the
    /// [`Test`] to be considered passing.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(test.max_deviance(), 0.01);
    /// ```
    pub fn max_deviance(&self) -> f64 {
        self.max_deviance
    }

    /// Runs a single experiment.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(1000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// test.run_single_experiment();
    /// assert_eq!(test.total_experiments(), 1);
    /// ```
    pub fn run_single_experiment(&mut self) -> Result<()> {
        // SAFETY: we hardcode generating one value, so we know this pop must unwrap.
        let data = self.provider.provide(1).pop().unwrap();

        let results = Experiment::<H, N>::try_new(self.build_hasher, data)
            .map_err(Error::Experiment)?
            .run(self.iterations_per_experiment);

        debug_assert_eq!(self.bit_flips.len(), results.len());

        for (i, value) in results.iter().enumerate() {
            self.bit_flips[i] += value;
        }

        self.total_experiments += 1;
        Ok(())
    }

    /// Generates a set of [`Results`] based on the current state of the
    /// [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::avalanche::sac::Test;
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::<RandomState, 64>::try_new(
    ///     &hasher,
    ///     Box::new(AlphanumericProvider::new(10)),
    ///     NonZeroUsize::try_from(100000).unwrap(),
    ///     0.01,
    /// )
    /// .unwrap();
    ///
    /// test.run_single_experiment();
    ///
    /// let results = test.results();
    /// // Do something with the results.
    /// ```
    pub fn results(&self) -> Results {
        let iterations = (self.total_experiments * self.iterations_per_experiment.get()) as f64;

        let bits = self
            .bit_flips
            .iter()
            .map(|flips| *flips as f64 / iterations)
            .enumerate()
            .map(|(i, value)| (i, OrderedFloat((value - 0.5).abs())))
            .collect::<Vec<_>>();

        let (index, max_bias) = bits
            .iter()
            .max_by_key(|&(_, value)| value)
            // SAFETY: there will be are least one result, as the number of iterations
            // per experiment is a `NonZeroUsize`. As such, there will always be a
            // maximum element, and this will always unwrap.
            .unwrap();

        tracing::info!("Max bias is bit {} with {:.2}%", index, max_bias * 100.0);

        Results {
            succeeded: *max_bias <= OrderedFloat(self.max_deviance),
            max_bias: (*index, *max_bias),
            bit_bias_offsets: bits,
        }
    }
}

impl<'a, H: BuildHasher, const N: usize> crate::r#trait::Test for Test<'a, H, N> {
    fn title(&self) -> &'static str {
        "Strict Avalanche Criterion"
    }

    fn report_section(&self) -> section::Test {
        let mut results = self.results();
        let visual = generate_visual_from_bits(&results.bit_bias_offsets);

        let (result, summary) = if results.succeeded {
            (
                module::Result::Pass,
                format!(
                    "The bias for every bit fell within a range of 0.5 ± {}.",
                    self.max_deviance
                ),
            )
        } else {
            (
                module::Result::Fail,
                format!(
                    "At least one bit had a bias that fell outside the range of 0.5 ± {}. See the \
                     bit bias profile and the most biased bits below for more information on \
                     which bits failed.",
                    self.max_deviance
                ),
            )
        };

        let mut details = format!(
            "{}\n\n{}\n\n{} => b <= 1% bias\n{} => b <= 5% bias\n{} => b  > 5% bias\n\nBit 0{}Bit \
             64\n{}\n\n{}\n",
            summary,
            "Bit Bias Profile".italic(),
            *ONE_PCT_CHAR,
            *FIVE_PCT_CHAR,
            *OTHER_PCT_CHAR,
            " ".repeat(55),
            visual,
            "Most Biased Bits".italic(),
        );

        results.bit_bias_offsets.sort_by_key(|(_, bias)| -*bias);
        for (index, bias_offset) in results.bit_bias_offsets.into_iter().take(10) {
            details.push_str(&format!(
                "\n* Index {:>2} had a bias offset of {:.2}%.",
                index,
                bias_offset * 100.0
            ));
        }

        get_report_base()
            .push_module(Module::new(
                result,
                "Strict Avalanche Criterion",
                None,
                Some(details),
            ))
            .try_build()
            .unwrap()
    }
}

/// Generates a visualization of which bits are biased (if any) from the bit
/// bias offset contained within a [`Results`].
fn generate_visual_from_bits(bit_bias_offsets: &[(usize, OrderedFloat<f64>)]) -> String {
    let mut visual = String::from("[");

    for (_, probability) in bit_bias_offsets.iter() {
        if probability <= &OrderedFloat(0.01) {
            visual.push_str(&format!("{}", &".".green()));
        } else if probability <= &OrderedFloat(0.05) {
            visual.push_str(&format!("{}", &"?".yellow()));
        } else {
            visual.push_str(&format!("{}", &"!".red()));
        }
    }

    visual.push(']');
    visual
}

/// Populates the boilerplate report information within a
/// [`Test`](section::Test).
pub fn get_report_base() -> section::test::Builder {
    let overview = "The Strict Avalanche Criterion (SAC) is a test to determine whether a hash \
                    function exhibits strong avalanching effects.\n\nBriefly, the avalanche \
                    effect is a desirable trait for a hash function whereby small changes in the \
                    input to the hash function cause significant changes in the output of the \
                    hash function. When a hash function does _not_ exhibit strong avalanching \
                    effects, it likely suffers from poor randomization, opening the door to \
                    various types of attacks.\n\nThe SAC introduces a formal test to ensure that \
                    a hash function exhibits sufficient avalanching. The core idea is this: when \
                    a single bit within the input data is randomly chosen and flipped, half of \
                    the hash's output bits for that input data should also change. Ideally, there \
                    won't be any bias as to which bits flip.";

    let algorithm =
        "For the hash function and data provider chosen, the algorithm runs multiple experiments. \
         For each experiment,\n\n(1) An array with a length matching the number of bits in the \
         output hash is initialized, and every element is set to 0. Each index in the array \
         represents a counter for the number of times that specific output bit flips during the \
         experiment.\n\n(2) A starting value for the input data is randomly generated.\n\n(3) For \
         a number of iterations, the following repeats:\n\n  * The hash current of the the input \
         data is computed (the 'prior hash').\n  * A single, random bit of the input data is \
         flipped.\n  * The hash of the new input data is computed (the 'new hash').\n  * Each bit \
         in the prior hash and the new hash are compared.\n    * Each bit that changes is \
         incremented by 1 in the tally array.\n\nThis process continues for a specified number of \
         experiment iterations.\n\nAfter all iterations have completed, the fraction of \
         iterations where each output bit flipped is calculated. In hash functions with strong \
         avlanching effects, each bit in the output should change roughly 50% of the time.";

    let interpretation = "* Each test has a set bias tolerance. For the test to pass, the bias \
                          for every output bit must fall within the range of the expected value \
                          (50%) ± the bias tolerance provided.\n\n* A bit bias profile is graphed \
                          below. This should give you a sense of which bits were biased and by \
                          what magnitude.\n\n* The most biased bits are also sorted in the \
                          respective section below. Use this list to determine the exact bias of \
                          the most biased bits.";

    Builder::default()
        .title("Strict Avalanche Criterion")
        .unwrap()
        .description(format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            "Overview".italic(),
            overview,
            "Algorithm".italic(),
            algorithm,
            "Interpretation".italic(),
            interpretation
        ))
        .unwrap()
}
