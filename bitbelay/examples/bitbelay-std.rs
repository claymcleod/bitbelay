//! A `bitbelay` command line tool for evaluating the Rust standard library's
//! hasher (via [`RandomState`]).
//!
//! To run: `cargo run --example bitbelay-std --features=cli`

use std::hash::RandomState;

pub fn main() -> anyhow::Result<()> {
    bitbelay::cli::wrapper(RandomState::default())
}
