//! Performance tests.

use std::hash::BuildHasher;

use bitbelay_report::section;

use crate::r#trait::Test as _;

pub mod speed;

/// A type of performance test.
#[derive(Debug)]
pub enum Test<'a, H: BuildHasher> {
    /// Speed test.
    Speed(speed::Test<'a, H>),
}

impl<'a, H: BuildHasher> Test<'a, H> {
    /// Gets a reference to a [`speed::Test`] wrapped in [`Some`] if
    /// the [`Test`] is a [`Test::Speed`]. Else, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::Test;
    /// use bitbelay_tests::performance::speed;
    /// use byte_unit::Byte;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::Speed(speed::Test::new(
    ///     &hasher,
    ///     provider,
    ///     "10 KiB".parse::<Byte>().unwrap(),
    ///     0.05,
    /// ));
    ///
    /// assert!(matches!(test.as_speed_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_speed_test(&self) -> Option<&speed::Test<'a, H>> {
        match self {
            Test::Speed(test) => Some(test),
        }
    }

    /// Consumes the [`Test`] and returns a [`speed::Test`] wrapped in
    /// [`Some`] if the [`Test`] is a [`Test::Speed`]. Else, returns
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::hash::RandomState;
    /// use std::num::NonZeroUsize;
    ///
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::Test;
    /// use bitbelay_tests::performance::speed;
    /// use byte_unit::Byte;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::Speed(speed::Test::new(
    ///     &hasher,
    ///     provider,
    ///     "10 KiB".parse::<Byte>().unwrap(),
    ///     0.05,
    /// ));
    ///
    /// assert!(matches!(test.into_speed_test(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_speed_test(self) -> Option<speed::Test<'a, H>> {
        match self {
            Test::Speed(test) => Some(test),
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
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    /// use bitbelay_tests::performance::Test;
    /// use bitbelay_tests::performance::speed;
    /// use byte_unit::Byte;
    ///
    /// let provider = Box::new(AlphanumericProvider::new(10));
    ///
    /// let hasher = RandomState::new();
    /// let mut test = Test::Speed(speed::Test::new(
    ///     &hasher,
    ///     provider,
    ///     "10 KiB".parse::<Byte>().unwrap(),
    ///     0.05,
    /// ));
    ///
    /// let section = test.report_section();
    /// // Do something with `section`.
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn report_section(&self) -> section::Test {
        match self {
            Test::Speed(test) => test.report_section(),
        }
    }
}
