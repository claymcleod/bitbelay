//! Example of wrapping a (really terrible) hash function using `bitbelay`.
//!
//! To run: `cargo run --example bitbelay-custom --features=cli`

/// A hasher that always returns `42`.
pub struct FortyTwoHasher;

impl std::hash::Hasher for FortyTwoHasher {
    fn finish(&self) -> u64 {
        42
    }

    fn write(&mut self, _: &[u8]) {
        // Noop.
    }
}

/// A [`std::hash::BuildHasher`] for [`FortyTwoHasher`].
#[derive(Default)]
pub struct RandomState;

impl std::hash::BuildHasher for RandomState {
    type Hasher = FortyTwoHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FortyTwoHasher
    }
}

pub fn main() -> anyhow::Result<()> {
    bitbelay::cli::wrapper(RandomState::default())
}
