//! Facilities for building your own CLI tools based on `bitbelay`.

pub mod commands;

use std::hash::BuildHasher;

use bitbelay_providers::AvailableProviders;
use clap::Parser;
use clap::Subcommand;

use crate::commands::avalanche;
use crate::commands::chi_squared;
use crate::commands::correlation;
use crate::commands::performance;

/// A performance evaluation harness for hashing functions.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The command to run.
    #[clap(subcommand)]
    command: Commands,

    /// The data provider.
    #[clap(short, long, global = true, default_value_t)]
    provider: AvailableProviders,

    /// Sets the log level to `TRACE`.
    #[clap(short, long, global = true)]
    trace: bool,

    /// Sets the log level to `INFO`.
    #[clap(short, long, global = true)]
    verbose: bool,
}

/// Commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Runs the avalanche test suite.
    Avalanche(commands::avalanche::Args),

    /// Runs the chi-squared test suite.
    ChiSquared(commands::chi_squared::Args),

    /// Runs the correlation test suite.
    Correlation(commands::correlation::Args),

    /// Runs the speed test suite.
    Performance(commands::performance::Args),
}

/// The main function for the wrapper.
fn main<H: BuildHasher>(build_hasher: H) -> anyhow::Result<()> {
    let global_args = Args::parse();

    let log_level = if global_args.trace {
        tracing::Level::TRACE
    } else if global_args.verbose {
        tracing::Level::INFO
    } else {
        tracing::Level::ERROR
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();
    tracing::info!("Hasher: {}.", std::any::type_name::<H>());
    tracing::info!("Provider: {}.", global_args.provider);

    match global_args.command {
        Commands::Avalanche(args) => {
            avalanche::main(args, build_hasher, global_args.provider.into())
        }
        Commands::ChiSquared(args) => {
            chi_squared::main(args, build_hasher, global_args.provider.into())
        }
        Commands::Correlation(args) => {
            correlation::main::<H, 64>(args, build_hasher, global_args.provider.into())
        }
        Commands::Performance(args) => {
            if global_args.trace || global_args.verbose {
                tracing::warn!("");
                tracing::warn!("***********");
                tracing::warn!("* WARNING *");
                tracing::warn!("***********");
                tracing::warn!("");
                tracing::warn!("The `--verbose` and `--trace` options should not ");
                tracing::warn!("generally be used with this command, as those options");
                tracing::warn!("can affect timing! Please don't report any results");
                tracing::warn!("with those settings enabled.");
                tracing::warn!("");
            };

            performance::main(args, build_hasher, global_args.provider.into())
        }
    }
}

/// A wrapper for an out-of-the-box command line tool for `bitbelay`.
pub fn wrapper<H: BuildHasher>(build_hasher: H) -> anyhow::Result<()> {
    main(build_hasher)
}
