//! A command for running the correlation test suite.

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use bitbelay_providers::Provider;
use bitbelay_report::Config;
use bitbelay_suites::r#trait::Suite;
use bitbelay_tests::correlation::bitwise;
use clap::ArgAction;
use colored::Colorize as _;

/// The first colorstop for a printed correlation matrix.
const CORRELATION_MATRIX_STOP_ONE: f64 = 0.05;

/// The second colorstop for a printed correlation matrix.
const CORRELATION_MATRIX_STOP_TWO: f64 = 0.25;

/// The third colorstop for a printed correlation matrix.
const CORRELATION_MATRIX_STOP_THREE: f64 = 0.50;

/// Arguments for the correlation command.
#[derive(clap::Args, Debug)]
pub struct Args {
    /// The number of iterations to carry out for the Bitwise test.
    #[arg(short, long, default_value_t = 1 << 16)]
    iterations: usize,

    /// The threshold of correlation at which any non-diagonal value causes the
    /// test to fail.
    #[arg(long, default_value_t = 0.05)]
    threshold: f64,

    /// Prints the full correlation matrix to the terminal.
    #[clap(long, action = ArgAction::SetTrue)]
    correlation_matrix: bool,

    /// Sets the width of each cell in the correlation matrix.
    #[clap(long, default_value_t = 2)]
    correlation_matrix_cell_width: usize,
}

/// The main function for the correlation command.
pub fn main<H: BuildHasher, const N: usize>(
    args: Args,
    build_hasher: H,
    provider: Box<dyn Provider>,
) -> anyhow::Result<()> {
    tracing::info!("Starting correlation test suite.");

    let iterations = NonZeroUsize::try_from(args.iterations)
        .map_err(|_| anyhow!("--iterations per experiment must be non-zero!"))?;

    let threshold = if (0.0..=1.0).contains(&args.threshold) {
        args.threshold
    } else {
        bail!("--threshold must be between 0.0 and 1.0!");
    };

    let correlation_matrix_cell_width = NonZeroUsize::try_from(args.correlation_matrix_cell_width)
        .map_err(|_| anyhow!("--correlation-matrix-cell-width must be non-zero!"))
        .and_then(|cell_size| {
            if cell_size.get() >= 2 {
                Ok(cell_size)
            } else {
                Err(anyhow!(
                    "--correlation-matrix-cell-width must be 2 or greater!"
                ))
            }
        })?;

    let mut suite = bitbelay_suites::correlation::suite::Builder::<H>::default()
        .build_hasher(&build_hasher)?
        .try_build::<N>()?;

    suite
        .run_bitwise_test(provider, iterations, threshold)
        .with_context(|| "running bitwise test")?;

    suite
        .report()
        .write_to(&mut std::io::stderr(), &Config::default())?;

    if args.correlation_matrix {
        // SAFETY: this first test should always be a bitwise test based on the order of
        let mut bitwise_tests = suite
            .tests()
            .iter()
            .filter_map(|test| test.as_bitwise_test())
            .collect::<Vec<_>>();

        match bitwise_tests.len() {
            0 => bail!(
                "there should be at least one bitwise test! This is an issue and should be looked \
                 at by the developers (please report this issue!)"
            ),
            1 => print_correlation_table::<N>(
                correlation_matrix_cell_width.get(),
                // SAFETY: for the first unwrap, we just checked to ensure there is exactly one
                // bitwise test, so this will always unwrap.
                //
                // SAFETY: for the second unwrap, this command _requires_ that at least
                // one test iteration is run. As such, this will always unwrap.
                bitwise_tests.pop().unwrap().results().unwrap(),
            ),
            v => bail!(
                "there are {} bitwise tests, and it's not clear what correlation matrix to print \
                 (please report this issue!)",
                v
            ),
        }
    }
    Ok(())
}

/// Prints a correlation table to stdout.
fn print_correlation_table<const N: usize>(width: usize, correlations: bitwise::Results) {
    if width == 0 {
        panic!("width of correlation table entries cannot be 0!");
    }

    // Print the legend.
    println!("{}", "Legend".bold().underline());

    println!(
        "{} => a correlation value in the range [0.0, {}]",
        " ".on_black(),
        CORRELATION_MATRIX_STOP_ONE
    );
    println!(
        "{} => a correlation value in the range ({}, {}]",
        " ".on_red(),
        CORRELATION_MATRIX_STOP_ONE,
        CORRELATION_MATRIX_STOP_TWO
    );
    println!(
        "{} => a correlation value in the range ({}, {}]",
        " ".on_yellow(),
        CORRELATION_MATRIX_STOP_TWO,
        CORRELATION_MATRIX_STOP_THREE
    );

    println!(
        "{} => a correlation value in the range [{}, 1.0]",
        " ".on_green(),
        CORRELATION_MATRIX_STOP_THREE,
    );

    println!();

    // Print the header.
    for i in 1..=N {
        if i < 10 || i % 10 == 0 {
            print!("{:^width$}", i.to_string().bold().underline())
        } else {
            print!("{:^width$}", i % 10, width = width);
        };
    }

    println!();

    // Print each correlation value.
    for i in 0..N {
        for j in 0..N {
            // SAFETY: for the first unwrap, due to the construction of this [`HashMap`]
            // always containing correlations of NxN size, this will always
            // unwrap.
            let value = *correlations.get(&(i, j)).unwrap();
            let cell = " ".repeat(width);

            if let Some(value) = value {
                // The correlation was able to be computed.
                if value > CORRELATION_MATRIX_STOP_THREE {
                    print!("{}", cell.on_green());
                } else if value > CORRELATION_MATRIX_STOP_TWO {
                    print!("{}", cell.on_yellow());
                } else if value > CORRELATION_MATRIX_STOP_ONE {
                    print!("{}", cell.on_red());
                } else {
                    print!("{}", cell.on_black());
                };
            } else {
                // The correlation was not able to be computed for some reason.
                print!("{}", cell.on_bright_purple());
            }
        }

        println!();
    }
}
