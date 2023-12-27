//! A `bitbelay` command line tool for evaluating [`ahash`], the default hashing
//! function used in [`hashbrown`](https://crates.io/crates/hashbrown).
//!
//! To run: `cargo run --example bitbelay-ahash --features=cli,hash-ahash`

type Hasher = ahash::RandomState;

use bitbelay::cli::wrapper;

pub fn main() -> anyhow::Result<()> {
    wrapper(Hasher::default())
}
