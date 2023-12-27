//! Runs a chi-squared test against Rust's default hasher.
//!
//! Example: `cargo run --release --example chi_squared`

use std::hash::RandomState;
use std::num::NonZeroUsize;

use bitbelay_providers::ascii::AlphanumericProvider;
use bitbelay_report::Config;
use bitbelay_suites::chi_squared::suite;
use bitbelay_suites::r#trait::Suite as _;

/// The main function.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hasher = RandomState::new();

    let mut suite = suite::Builder::default()
        .build_hasher(&hasher)?
        .try_build()?;

    let provider = Box::new(AlphanumericProvider::new(10));

    suite.run_goodness_of_fit(provider, NonZeroUsize::try_from(10_000).unwrap(), 0.05);
    suite
        .report()
        .write_to(&mut std::io::stderr(), &Config::default())?;

    Ok(())
}
