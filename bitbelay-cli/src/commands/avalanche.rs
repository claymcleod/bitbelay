//! A command for running the avalanche test suite.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use anyhow::Context;
use anyhow::anyhow;
use anyhow::bail;
use bitbelay_providers::Provider;
use bitbelay_report::Config;
use bitbelay_suites::r#trait::Suite;

/// Arguments for the avalanche command.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// The number of experiments to perform for the Strict Avalanche Criterion
    /// test.
    #[arg(short, long, default_value_t = 1 << 12)]
    experiments: usize,

    /// The number of iterations per experiment for the Strict Avalanche
    /// Criterion test.
    #[arg(short, long, default_value_t = 1 << 12)]
    iterations_per_experiment: usize,

    /// The maximum deviance that any single bit can have off of 50% bias for
    /// the test to be considered successful.
    #[arg(short, long, default_value_t = 0.01)]
    max_deviance: f64,
}

/// The main function for the avalanche command.
pub fn main<H: BuildHasher>(
    args: Args,
    build_hasher: H,
    provider: Box<dyn Provider>,
) -> anyhow::Result<()> {
    tracing::info!("Starting avalanche test suite.");

    let experiments = NonZeroUsize::try_from(args.experiments)
        .map_err(|_| anyhow!("--experiments must be non-zero!"))?;

    let iterations_per_experiment = NonZeroUsize::try_from(args.iterations_per_experiment)
        .map_err(|_| anyhow!("--iterations per experiment must be non-zero!"))?;

    let max_deviance = if (0.0..=1.0).contains(&args.max_deviance) {
        args.max_deviance
    } else {
        bail!("--max-deviance must be in the range of [0, 1]!")
    };

    let mut suite = bitbelay_suites::avalanche::suite::Builder::<H, 64>::default()
        .build_hasher(&build_hasher)?
        .try_build()?;

    suite
        .run_strict_avalanche_criterion_test(
            provider,
            experiments,
            iterations_per_experiment,
            max_deviance,
        )
        .with_context(|| "running strict avalanche criterion test")?;

    suite
        .report()
        .write_to(&mut std::io::stderr(), &Config::default())?;

    Ok(())
}
