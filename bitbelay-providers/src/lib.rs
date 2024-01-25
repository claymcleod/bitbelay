//! Data providers for `bitbelay`.

use clap::ValueEnum;

pub mod ascii;
pub mod numeric;

/// The number of bits for a _short_ length data provider.
const SHORT_BITS: usize = 3;

/// The number of bits for a _medium_ length data provider.
const MEDIUM_BITS: usize = 6;

/// The number of bits for a _long_ length data provider.
const LONG_BITS: usize = 12;

/// A data provider for a hash function.
pub trait Provider: std::fmt::Debug {
    /// The name of the provider.
    fn name(&self) -> &str;

    /// Provides data by specifying the number of desired results (not bytes).
    fn provide(&mut self, n: usize) -> Vec<&[u8]>;

    /// The number of bytes per data provided.
    fn bytes_per_input(&mut self) -> usize;
}

/// A list of possible providers.
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum AvailableProviders {
    /// A medium ASCII alphanumeric string.
    #[clap(name = "ascii-alphanumeric")]
    #[default]
    ASCIIAlphanumeric,

    /// A long ASCII alphanumeric string.
    #[clap(name = "ascii-alphanumeric-long")]
    ASCIIAlphanumericLong,

    /// A short ASCII alphanumeric string.
    #[clap(name = "ascii-alphanumeric-short")]
    ASCIIAlphanumericShort,

    /// A medium array of `u64`s.
    #[clap(name = "u64")]
    U64,

    /// A long array of `u64`s.
    #[clap(name = "u64-long")]
    U64Long,

    /// A short array of `u64`s.
    #[clap(name = "u64-short")]
    U64Short,
}

impl std::fmt::Display for AvailableProviders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailableProviders::ASCIIAlphanumeric => write!(f, "ascii-alphanumeric"),
            AvailableProviders::ASCIIAlphanumericLong => write!(f, "ascii-alphanumeric-long"),
            AvailableProviders::ASCIIAlphanumericShort => write!(f, "ascii-alphanumeric-short"),
            AvailableProviders::U64 => write!(f, "u64"),
            AvailableProviders::U64Long => write!(f, "u64-long"),
            AvailableProviders::U64Short => write!(f, "u64-short"),
        }
    }
}

impl From<AvailableProviders> for Box<dyn Provider> {
    fn from(provider: AvailableProviders) -> Self {
        match provider {
            // ASCII alphanumeric-based providers.
            AvailableProviders::ASCIIAlphanumeric => {
                Box::new(ascii::AlphanumericProvider::new(1 << MEDIUM_BITS))
            }
            AvailableProviders::ASCIIAlphanumericLong => {
                Box::new(ascii::AlphanumericProvider::new(1 << LONG_BITS))
            }
            AvailableProviders::ASCIIAlphanumericShort => {
                Box::new(ascii::AlphanumericProvider::new(1 << SHORT_BITS))
            }

            // `u64`-based providers.
            AvailableProviders::U64 => {
                Box::new(numeric::Unsigned64BitProvider::new(1 << MEDIUM_BITS))
            }
            AvailableProviders::U64Long => {
                Box::new(numeric::Unsigned64BitProvider::new(1 << LONG_BITS))
            }
            AvailableProviders::U64Short => {
                Box::new(numeric::Unsigned64BitProvider::new(1 << SHORT_BITS))
            }
        }
    }
}
