//! Experiments within a Strict Avalanche Criterion test.

use core::hash::BuildHasher;
use std::num::NonZeroUsize;

use bitvec::prelude::*;
use rand::distributions::Distribution as _;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

/// An error related to an [`Experiment`].
#[derive(Debug)]
pub enum Error {
    /// Attempted to create an empty [`Experiment`].
    EmptyData,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyData => write!(f, "empty data"),
        }
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) with an [`Error`].
type Result<T> = std::result::Result<T, Error>;

/// An experiment within a Strict Avalanche Criterion test.
#[derive(Debug)]
pub struct Experiment<'a, H: BuildHasher, const N: usize> {
    /// The build hasher.
    build_hasher: &'a H,

    /// The data being hashed.
    data: BitVec<u8, Lsb0>,

    /// The random number generator.
    rng: ThreadRng,
}

impl<'a, H: BuildHasher, const N: usize> Experiment<'a, H, N> {
    /// Attempts to create a new [`Experiment`].
    ///
    /// # Notes
    ///
    /// * If `data` is empty, an [`Error::EmptyData`] is thrown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_tests::avalanche::sac::Experiment;
    ///
    /// let hasher = RandomState::new();
    /// let mut experiment = Experiment::<RandomState, 64>::try_new(&hasher, b"Hello, world!")?;
    ///
    /// experiment.run(NonZeroUsize::try_from(10).unwrap());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_new<T: AsRef<[u8]>>(build_hasher: &'a H, data: T) -> Result<Self> {
        let data = data.as_ref();

        if data.is_empty() {
            return Err(Error::EmptyData);
        }

        Ok(Self {
            build_hasher,
            data: BitVec::<u8, Lsb0>::from_slice(data),
            rng: rand::thread_rng(),
        })
    }

    /// Gets a reference to the build hasher for this [`Experiment`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::BuildHasher as _;
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_tests::avalanche::sac::Experiment;
    ///
    /// let hasher = RandomState::new();
    /// let mut experiment = Experiment::<RandomState, 64>::try_new(&hasher, b"Hello, world!")?;
    ///
    /// // Used as a surrogate to test that the [`BuildHasher`]s are the same.
    /// assert_eq!(
    ///     experiment.build_hasher().hash_one("42"),
    ///     hasher.hash_one("42")
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn build_hasher(&self) -> &H {
        self.build_hasher
    }

    /// Gets a reference to inner data for this [`Experiment`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    ///
    /// use bitbelay_tests::avalanche::sac::Experiment;
    ///
    /// let hasher = RandomState::new();
    /// let mut experiment = Experiment::<RandomState, 64>::try_new(&hasher, b"Hello, world!")?;
    ///
    /// assert_eq!(experiment.data().as_raw_slice(), b"Hello, world!");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn data(&self) -> &BitVec<u8, Lsb0> {
        &self.data
    }

    /// Flips a random bit within `data`.
    fn flip_random_bit(&mut self) {
        let range = Uniform::from(0..self.data.len());
        let index = range.sample(&mut self.rng);
        let mut bit = self.data.get_mut(index).unwrap();
        *bit = !*bit;
    }

    /// Hashes the current value of `data` and returns the result.
    fn hash_data(&mut self) -> u64 {
        self.build_hasher.hash_one(self.data.as_raw_slice())
    }

    /// Runs the experiment with `iterations` iterations.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_tests::avalanche::sac::Experiment;
    ///
    /// let hasher = RandomState::new();
    /// let mut experiment = Experiment::<RandomState, 64>::try_new(&hasher, b"Hello, world!")?;
    ///
    /// experiment.run(NonZeroUsize::try_from(10).unwrap());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn run(&mut self, iterations: NonZeroUsize) -> [usize; N] {
        let mut previous = self.hash_data();
        let mut bit_changes = [0usize; N];

        for _ in 0..iterations.get() {
            self.flip_random_bit();

            let next = self.hash_data();
            let result = previous ^ next;

            #[allow(clippy::needless_range_loop)]
            for i in 0..N {
                if (result >> i) & 1 == 1 {
                    bit_changes[i] += 1;
                }
            }

            previous = next;
        }

        bit_changes
    }
}

#[cfg(test)]
mod tests {
    use std::hash::RandomState;

    use super::*;

    #[test]
    fn flipping_random_bits() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let build_hasher = RandomState::new();
        let mut experiment =
            Experiment::<RandomState, 64>::try_new(&build_hasher, b"Hello, world!")?;

        let mut old_number_of_ones = experiment.data().count_ones() as isize;

        for _ in 0..10000 {
            experiment.flip_random_bit();
            let new_number_of_ones = experiment.data().count_ones() as isize;
            assert_eq!((new_number_of_ones - old_number_of_ones).abs(), 1);
            old_number_of_ones = new_number_of_ones;
        }

        Ok(())
    }
}
