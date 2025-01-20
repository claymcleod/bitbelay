//! Correlation tests.

use std::hash::BuildHasher;

use bitbelay_report::section;

use crate::r#trait::Test as _;

pub mod bitwise;

/// A type of correlation test.
#[derive(Debug)]
pub enum Test<'a, H: BuildHasher, const N: usize> {
    /// Bitwise test.
    Bitwise(bitwise::Test<'a, H, N>),
}

impl<'a, H: BuildHasher, const N: usize> Test<'a, H, N> {
    /// Gets a reference to a [`bitwise::Test`] wrapped in [`Some`] if
    /// the [`Test`] is a [`Test::Bitwise`]. Else, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::correlation::Test;
    /// use bitbelay_tests::correlation::bitwise;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::Bitwise(bitwise::Test::<RandomState, 64>::new(&hasher, 0.05));
    /// assert!(matches!(test.as_bitwise_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_bitwise_test(&self) -> Option<&bitwise::Test<'a, H, N>> {
        match self {
            Test::Bitwise(test) => Some(test),
        }
    }

    /// Consumes the [`Test`] and returns a [`bitwise::Test`] wrapped in
    /// [`Some`] if the [`Test`] is a [`Test::Bitwise`].
    /// Else, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::correlation::Test;
    /// use bitbelay_tests::correlation::bitwise;
    ///
    /// let hasher = RandomState::new();
    /// let test = Test::Bitwise(bitwise::Test::<RandomState, 64>::new(&hasher, 0.05));
    /// assert!(matches!(test.into_bitwise_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_bitwise_test(self) -> Option<bitwise::Test<'a, H, N>> {
        match self {
            Test::Bitwise(test) => Some(test),
        }
    }

    /// Generates a report section for the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::Provider;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::correlation::Test;
    /// use bitbelay_tests::correlation::bitwise;
    ///
    /// let mut provider: Box<dyn Provider> = Box::new(AlphanumericProvider::new(10));
    /// let hasher = RandomState::new();
    /// let mut test = bitwise::Test::<RandomState, 64>::new(&hasher, 0.05);
    /// test.run(&mut provider, NonZeroUsize::try_from(10).unwrap());
    ///
    /// let mut test = Test::Bitwise(test);
    /// let results = test.report_section();
    /// // Include the section in a report.
    /// ```
    pub fn report_section(&self) -> section::Test {
        match self {
            Test::Bitwise(test) => test.report_section(),
        }
    }
}
