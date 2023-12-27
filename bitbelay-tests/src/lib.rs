//! Individual tests within `bitbelay`.

pub mod avalanche;
pub mod chi_squared;
pub mod correlation;
pub mod performance;

/// Traits for `bitbelay` tests.
pub mod r#trait {
    use bitbelay_report::section;

    /// A test.
    pub trait Test {
        /// Gets the name of the test.
        fn title(&self) -> &'static str;

        /// Gets the report from the test suite.
        fn report_section(&self) -> section::Test;
    }
}
