#[cfg(not(feature = "hash-ahash"))]
use std::hash::RandomState as Hasher;
#[cfg(feature = "hash-ahash")]
type Hasher = ahash::RandomState;

use bitbelay::cli::wrapper;

pub fn main() -> anyhow::Result<()> {
    wrapper(Hasher::default())
}
