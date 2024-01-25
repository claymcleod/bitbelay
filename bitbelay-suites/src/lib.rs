//! Test suites for `bitbelay`.
//!
//! This crate is comprised of three different entities:
//!
//! * **Suites** are collections of tests related to a particular topic—they are
//!   instantiated with a particular hasher and have methods to run different
//!   tests within the context of the suite. Last they keep track of all of the
//!   tests run and generate a report.
//! * **Tests** are specific exercises within the context of a suite. A suite
//!   may have many supported tests and can run any combination of tests (e.g.,
//!   all available tests of the same type of input data or the same test on
//!   multiple types of input data).
//! * **Modules** are the discrete components of each test that are pass/fail.
//!
//! For example, the Chi-squared **suite** supports running a goodness of fit
//! **test** for a particular input. Underneath the goodness of fit test, the
//! failure to reject the null hypothesis is a **module** that can be passed or
//! failed.
//!
//! We considerd this structure a balance between allowing flexibility within
//! the provided facilities of this crate while also making the concepts clear
//! to implement.

pub mod avalanche;
pub mod chi_squared;
pub mod correlation;
pub mod performance;

/// Traits for `bitbelay` test suites.
pub mod r#trait {
    use bitbelay_report::Report;

    /// A suite of curated tests designed for a particular purpose.
    pub trait Suite {
        /// Gets the name of the test suite.
        fn title(&self) -> &'static str;

        /// Gets the report from the test suite.
        fn report(&self) -> Report;
    }
}
