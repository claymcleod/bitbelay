//! A command for running the speed test suite.

use std::fmt::Debug;
use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use anyhow::anyhow;
use bitbelay_providers::Provider;
use bitbelay_report::Config;
use bitbelay_suites::performance;
use bitbelay_suites::r#trait::Suite as _;
use byte_unit::Byte;

/// Arguments for the avalanche command.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// The amount of data to generate.
    #[clap(short, long, default_value = "10 MB")]
    data_size: String,

    /// The number of iterations to complete.
    #[arg(short, long, default_value_t = 1 << 8)]
    iterations: usize,

    /// The threshold needed for the speed test to pass in megabytes per second.
    #[arg(short, long, default_value_t = 1000.0)]
    threshold: f64,
}

/// The main function for the speed command.
pub fn main<H: BuildHasher>(
    args: Args,
    build_hasher: H,
    provider: Box<dyn Provider>,
) -> anyhow::Result<()> {
    tracing::info!("Starting speed test suite.");

    let iterations = NonZeroUsize::try_from(args.iterations)
        .map_err(|_| anyhow!("--iterations must be non-zero!"))?;

    let desired_data_size = args
        .data_size
        .parse::<Byte>()
        .map_err(|_| anyhow!("invalid data size: {}", args.data_size))?;

    let mut suite = performance::suite::Builder::default()
        .build_hasher(&build_hasher)
        .unwrap()
        .try_build()
        .unwrap();

    suite.run_speed_test(provider, iterations, desired_data_size, args.threshold)?;

    suite
        .report()
        .write_to(&mut std::io::stderr(), &Config::default())?;

    Ok(())
}
