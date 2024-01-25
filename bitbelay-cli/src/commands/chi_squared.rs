//! A command for running the chi-squared test suite.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use anyhow::anyhow;
use anyhow::bail;
use bitbelay_providers::Provider;
use bitbelay_report::Config;
use bitbelay_suites::chi_squared::suite::Builder;
use bitbelay_suites::r#trait::Suite as _;
use tracing::Level;

/// The default number of iterations per bucket.
///
/// NOTE: if this changes, update the argument documentation for `iterations`.
const DEFAULT_ITERATIONS_PER_BUCKET: usize = 1000;

/// Arguments for the chi-squared command.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// The number of buckets.
    #[arg(short, long, default_value_t = 64)]
    buckets: usize,

    /// The number of iterations to test.
    ///
    /// If no number is given, then the number will be 100 * the number of
    /// buckets to ensure enough samples are taken.
    #[arg(short, long)]
    iterations: Option<usize>,

    /// The threshold of statistical significance.
    #[arg(long, default_value_t = 0.05)]
    threshold: f64,
}

/// The main function for the chi-squared command.
pub fn main<H: BuildHasher>(
    args: Args,
    build_hasher: H,
    provider: Box<dyn Provider>,
) -> anyhow::Result<()> {
    tracing::info!("Starting chi-squared test suite.");

    let buckets =
        NonZeroUsize::try_from(args.buckets).map_err(|_| anyhow!("--buckets must be non-zero!"))?;

    let iterations = NonZeroUsize::try_from(
        args.iterations
            .unwrap_or(args.buckets * DEFAULT_ITERATIONS_PER_BUCKET),
    )
    .map_err(|_| anyhow!("--iterations must be non-zero!"))?;

    if !(0.0..=1.0).contains(&args.threshold) {
        bail!("--threshold must be between 0.0 and 1.0!");
    }

    tracing::info!(
        "Running chi-squared test with {} buckets for {} iterations.",
        args.buckets,
        iterations
    );

    let mut suite = Builder::default()
        .buckets(buckets)
        .unwrap()
        .build_hasher(&build_hasher)
        .unwrap()
        .try_build()
        .unwrap();

    suite.run_goodness_of_fit(provider, iterations, args.threshold);

    if tracing::enabled!(Level::TRACE) {
        // SAFETY: we know there must be one test because we just ran it above!
        let test = suite.tests().last().unwrap();
        for (i, entries) in test
            .as_goodness_of_fit_test()
            // SAFETY: we also know that the last test was a goodness of fit test.
            .unwrap()
            .buckets()
            .iter()
            .enumerate()
        {
            tracing::trace!("[Bucket {}] => {}", i + 1, entries);
        }
    }

    suite
        .report()
        .write_to(&mut std::io::stderr(), &Config::default())?;

    Ok(())
}
