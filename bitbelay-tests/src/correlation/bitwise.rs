//! Bitwise correlation test.

use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hasher as _;
use std::num::NonZeroUsize;

use bitbelay_providers::Provider;
use bitbelay_report::section;
use bitbelay_report::section::test;
use bitbelay_report::section::test::module;
use bitbelay_statistics::correlation::pearson;
use colored::Colorize as _;
use ordered_float::OrderedFloat;
use tracing::debug;
use tracing::info;

/// Results from a bitwise correlation test.
pub type Results = HashMap<(usize, usize), Option<f64>>;

/// A bitwise correlation test.
#[derive(Debug)]
pub struct Test<'a, H: BuildHasher, const N: usize> {
    /// The build hasher.
    build_hasher: &'a H,

    /// The bit values accumulated for each bit in the output hash.
    bit_values: [Vec<f64>; N],

    /// The threshold of correlation at which any non-diagonal value causes the
    /// test to fail.
    threshold: f64,
}

impl<'a, H: BuildHasher, const N: usize> Test<'a, H, N> {
    /// Creates a new [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_tests::correlation::bitwise::Test;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::<RandomState, 64>::new(&hasher, 0.05);
    /// ```
    pub fn new(build_hasher: &'a H, threshold: f64) -> Self {
        Self {
            build_hasher,
            bit_values: [(); N].map(|_| Vec::new()),
            threshold,
        }
    }

    /// Gets the bit values of this [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::correlation::bitwise::Test;
    ///
    /// let mut provider: Box<dyn Provider> = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let mut test = Test::<RandomState, 64>::new(&hasher, 0.05);
    ///
    /// test.run(&mut provider, NonZeroUsize::try_from(10).unwrap());
    ///
    /// assert_eq!(test.bit_values()[0].len(), 10);
    /// ```
    pub fn bit_values(&self) -> &[Vec<f64>; N] {
        &self.bit_values
    }

    /// Gets the threshold of this [`Test`] by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_tests::correlation::bitwise::Test;
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::<RandomState, 64>::new(&hasher, 0.05);
    ///
    /// assert_eq!(test.threshold(), 0.05);
    /// ```
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Runs a set of iterations using a [`Provider`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_providers::numeric::Unsigned64BitProvider;
    /// use bitbelay_tests::correlation::bitwise::Test;
    ///
    /// let mut alphas: Box<dyn Provider> = Box::new(AlphanumericProvider::new(10));
    /// let mut numbers: Box<dyn Provider> = Box::new(Unsigned64BitProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::<RandomState, 64>::new(&hasher, 0.05);
    ///
    /// test.run(&mut alphas, NonZeroUsize::try_from(10).unwrap());
    /// test.run(&mut numbers, NonZeroUsize::try_from(10).unwrap());
    ///
    /// assert_eq!(test.bit_values()[0].len(), 20);
    /// ```
    pub fn run(&mut self, provider: &mut Box<dyn Provider>, iterations: NonZeroUsize) {
        let hashes = compute_hashes(self.build_hasher, provider, iterations);
        let newly_computed_bit_values = extract_bit_values_from_hashes::<u64, N>(&hashes);

        for (i, values) in newly_computed_bit_values.iter().enumerate() {
            // SAFETY: the length of the `newly_computed_bit_values` array is statically
            // guarenteed to be `N`, which is the same as the size of `self.bit_values`.
            // As such, this indexing will always succeed.
            self.bit_values[i].extend(values);
        }
    }

    /// Gets the [`Results`] of a [`Test`] using the [`Test`]'s current interal
    /// state.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::correlation::bitwise::Test;
    ///
    /// let mut provider: Box<dyn Provider> = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let mut test = Test::<RandomState, 64>::new(&hasher, 0.05);
    ///
    /// test.run(&mut provider, NonZeroUsize::try_from(10).unwrap());
    ///
    /// let results = test.results().unwrap();
    /// // Do something with the results.
    /// ```
    pub fn results(&self) -> Option<Results> {
        info!("Computing Pearson correlations for each bit-bit mapping.");

        // SAFETY: there should always be at least one output bit, so this should always
        // unwrap.
        if self.bit_values.first().unwrap().is_empty() {
            return None;
        }

        let mut results = HashMap::new();

        for i in 0..N {
            for j in 0..N {
                // SAFETY: we checked above that there was at least test iteration run. As
                // such, this should always unwrap.
                let correlation = pearson::correlation(&self.bit_values[i], &self.bit_values[j]);
                results.insert((i, j), correlation);
            }

            debug!("Computed all bit-bit correlations for bit index {}.", i);
        }

        Some(results)
    }
}

impl<'a, H: BuildHasher, const N: usize> crate::r#trait::Test for Test<'a, H, N> {
    fn title(&self) -> &'static str {
        "Bitwise Correlation"
    }

    fn report_section(&self) -> bitbelay_report::section::Test {
        // SAFETY: there should always be at least one output bit, so this should always
        // unwrap.
        if self.bit_values.first().unwrap().is_empty() {
            panic!("a report can only be generated when at least one test has been run!");
        }

        let mut correlations = self
            .results()
            // SAFETY: we checked above that there was at least test iteration run. As
            // such, this should always unwrap.
            .unwrap()
            .into_iter()
            .filter(|((i, j), _)| i != j)
            .map(|(pos, corr)| {
                (
                    pos,
                    OrderedFloat(match corr {
                        Some(corr) => corr.abs(),
                        None => panic!(
                            "one of the correlations was [`None`], which means that there was \
                             some issue computing the correlation metrics. This is especially \
                             possible when there were not many iterations run: for example, if \
                             every bit in one of the arrays was a 0. We recommend trying the test \
                             again with more samples. If this continue to happen, please file an \
                             issue."
                        ),
                    }),
                )
            })
            .collect::<Vec<_>>();
        correlations.sort_by(|(_, a), (_, b)| b.cmp(a));

        let (result, mut details) = if correlations
            .iter()
            .any(|(_, correlation)| correlation.into_inner() >= self.threshold)
        {
            (
                module::Result::Fail,
                String::from(
                    "One or more non-diagonals had a correlation greater than or equal to the \
                     threshold.\n\n",
                ),
            )
        } else {
            (
                module::Result::Pass,
                String::from("All non-diagonals had a correlation lower than the threshold.\n\n"),
            )
        };

        details.push_str("Maxmium correlation values:\n");

        correlations
            .iter()
            .take(10)
            .map(|((i, j), correlation)| format!("\n  * ({}, {}) => {:.4}", i, j, correlation))
            .for_each(|s| details.push_str(&s));

        let module =
            module::Module::new(result, "Pearson correlation threshold", None, Some(details));
        get_report_base().push_module(module).try_build().unwrap()
    }
}

/// Computes `iterations` number of hashes using the hasher provided in
/// `build_hasher` and the data provided by `provider`.
fn compute_hashes<H: BuildHasher>(
    build_hasher: &H,
    provider: &mut Box<dyn Provider>,
    iterations: NonZeroUsize,
) -> Vec<u64> {
    info!("Computing {} hashes.", iterations);

    provider
        .provide(iterations.get())
        .into_iter()
        .enumerate()
        .map(|(i, input)| {
            if i % 1_000 == 0 && i > 0 {
                debug!("Computed {} hashes.", i);
            }

            let mut hasher = build_hasher.build_hasher();
            hasher.write(input);
            hasher.finish()
        })
        .collect()
}

/// Extracts each bit within a set of hashes to a [`Vec`] of their own.
///
/// For example, bit 0 from every hash is pulled into the first [`Vec<f64>`]
/// returned, bit 1 from every hash is pulled into the second [`Vec<f64>`]
/// return, etc.
fn extract_bit_values_from_hashes<T, const N: usize>(hashes: &[T]) -> Vec<Vec<f64>>
where
    T: Copy
        + std::ops::Shr<usize, Output = T>
        + std::ops::BitAnd<Output = T>
        + From<u64>
        + Into<u64>,
{
    info!(
        "Extracing bit values across {} {}-bit hashes.",
        hashes.len(),
        N
    );

    let mut bit_values: Vec<Vec<f64>> = vec![Vec::new(); N];

    for hash in hashes {
        for (i, bit_value) in bit_values.iter_mut().enumerate() {
            let as_byte = ((*hash >> i) & T::from(1_u64)).into();
            bit_value.push(as_byte as f64);
        }
    }

    bit_values
}

/// Populates the boilerplate report information within a
/// [`Test`](section::Test).
pub fn get_report_base() -> section::test::Builder {
    let overview = "The bitwise correlation test assesses the correlation between each pair of \
                    bits, known as 'bit-bit comparisons', for a set of output hashes using the \
                    Pearson correlation coefficient.";

    let algorithm =
        "For a specified hash function, a provider and number of iterations is specified:\n\n(1) \
         A random input is generated from the provider and the output hash is computed. This \
         happens for the number of iterations specified, and the results are accumulated in an \
         array. You can think of this as, roughly, a matrix of bits where each row is an output \
         hash and each column is the bit at position _i_ of the output hash.\n\n(2) This matrix \
         of bits is effectively transposed, meaning that an array is created for each bit \
         position, with each array containing the values of that bit across all hashes. These \
         arrays are referred to as 'bit values' for their respective bit positions.\n\n(3) For \
         each pair of bit positions, the Pearson correlation is calculated between their \
         corresponding arrays of bit values. This measures the level of correlation between every \
         pair of output bits. Note that, though Pearson correlation is symmetric (meaning the \
         correlation of (i, j) is the same as the correlation of (j, i)), all pairwise \
         comparisons are computed.\n\n(4) The resulting correlations are stored in a HashMap, \
         with each key being a tuple `(i, j)` representing the Pearson correlation coefficient \
         between the bit values at position _i_ and the bit values at position _j_.";

    let interpretation = "* A 'good' result is one where bits within the output hashes are not \
                          highly correlated with one another. This indicates that the bits are \
                          largely independent under the test.\n\n * There is one exception, \
                          called the 'diagonal' of the correlation matrix. The rationale behind \
                          this is straightforward: when an array of bit values at position _i_ is \
                          compared against itself, the arrays are identical, and the correlation \
                          should be 1.0. This presence of this phenomenon is often used as a \
                          check to ensure that results are being calculated as expected.";

    let sources = "* https://en.wikipedia.org/wiki/Pearson_correlation_coefficient";

    test::Builder::default()
        .title("Bitwise Pearson Correlation".to_string())
        .unwrap()
        .description(format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            "Overview".italic(),
            overview,
            "Algorithm".italic(),
            algorithm,
            "Interpretation".italic(),
            interpretation,
            "Sources".italic(),
            sources
        ))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::correlation::bitwise::extract_bit_values_from_hashes;

    #[test]
    fn it_extracts_bit_values_from_hashes_correctly() {
        let hashes: [u64; 4] = [0x00, 0x01, 0x02, 0x03];
        let bit_values = extract_bit_values_from_hashes::<u64, 64>(&hashes[..]);

        #[allow(clippy::needless_range_loop)]
        for i in 2..64 {
            assert_eq!(bit_values[i], &[0., 0., 0., 0.]);
        }

        assert_eq!(bit_values[1], &[0., 0., 1., 1.]);
        assert_eq!(bit_values[0], &[0., 1., 0., 1.]);
    }
}
