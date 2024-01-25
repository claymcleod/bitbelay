//! Builder for a [`Suite`].

use std::hash::BuildHasher;
use std::num::NonZeroUsize;

use crate::chi_squared::Suite;

/// The default number of buckets to use when none are provided.
const DEFAULT_BUCKETS: usize = 256;

/// An error when a required field is missing.
#[derive(Debug)]
pub enum MissingError {
    /// No build hasher was provided to the [`Builder`].
    BuildHasher,
}

impl std::fmt::Display for MissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingError::BuildHasher => write!(f, "build hasher"),
        }
    }
}

impl std::error::Error for MissingError {}

/// An error when multiple values are provided for a singular field.
#[derive(Debug)]
pub enum MultipleError {
    /// Multiple build hasher values were provided to the [`Builder`].
    BuildHasher,

    /// Multiple buckets values were provided to the [`Builder`].
    Buckets,
}

impl std::fmt::Display for MultipleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultipleError::BuildHasher => write!(f, "build hasher"),
            MultipleError::Buckets => write!(f, "buckets"),
        }
    }
}

impl std::error::Error for MultipleError {}

/// An error related to a [`Builder`].
#[derive(Debug)]
pub enum Error {
    /// A required field was missing from the [`Builder`].
    Missing(MissingError),

    /// Multiple values were provided for a singular field in the [`Builder`].
    Multiple(MultipleError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Missing(err) => write!(f, "missing error: {}", err),
            Error::Multiple(err) => write!(f, "multiple error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
type Result<T> = std::result::Result<T, Error>;

/// A builder for a [`Suite`].
#[derive(Debug)]
pub struct Builder<'a, H: BuildHasher> {
    /// The hash function builder.
    build_hasher: Option<&'a H>,

    /// The number of buckets to use within each test.
    buckets: Option<NonZeroUsize>,
}

impl<'a, H: BuildHasher> Default for Builder<'a, H> {
    fn default() -> Self {
        Self {
            build_hasher: Default::default(),
            buckets: Default::default(),
        }
    }
}

impl<'a, H: BuildHasher> Builder<'a, H> {
    /// Sets the number of buckets to use for tests within this [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::default()
    ///     .buckets(NonZeroUsize::try_from(2048).unwrap())?
    ///     .build_hasher(&hasher)?
    ///     .try_build()?;
    ///
    /// assert_eq!(suite.buckets().get(), 2048);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn buckets(mut self, buckets: NonZeroUsize) -> Result<Self> {
        if self.buckets.is_some() {
            return Err(Error::Multiple(MultipleError::Buckets));
        }

        self.buckets = Some(buckets);
        Ok(self)
    }

    /// Sets the [`BuildHasher`] for this [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::default().build_hasher(&hasher)?.try_build()?;
    ///
    /// // Used as a surrogate to test that the [`BuildHasher`]s are the same.
    /// assert_eq!(suite.build_hasher().hash_one("42"), hasher.hash_one("42"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn build_hasher(mut self, build_hasher: &'a H) -> Result<Self> {
        if self.build_hasher.is_some() {
            return Err(Error::Multiple(MultipleError::BuildHasher));
        }

        self.build_hasher = Some(build_hasher);
        Ok(self)
    }

    /// Consumes `self` to attempt to build a [`Suite`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_suites::chi_squared::suite::Builder;
    ///
    /// let hasher = RandomState::new();
    /// let suite = Builder::default()
    ///     .buckets(NonZeroUsize::try_from(2048).unwrap())?
    ///     .build_hasher(&hasher)?
    ///     .try_build()?;
    ///
    /// assert_eq!(suite.buckets().get(), 2048);
    /// // Used as a surrogate to test that the [`BuildHasher`]s are the same.
    /// assert_eq!(suite.build_hasher().hash_one("42"), hasher.hash_one("42"));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_build(self) -> Result<Suite<'a, H>> {
        let build_hasher = self
            .build_hasher
            .ok_or(Error::Missing(MissingError::BuildHasher))?;

        let buckets = self
            .buckets
            // SAFETY: [`DEFAULT_BUCKETS`] is manually crafted to be a non-zero usize.
            .unwrap_or(NonZeroUsize::try_from(DEFAULT_BUCKETS).unwrap());

        Ok(Suite {
            build_hasher,
            tests: Vec::new(),
            buckets,
        })
    }
}
