//! Sections within a [`Report`](super::Report).

pub mod test;

pub use test::Test;

/// A section within a report.
#[derive(Debug)]
pub enum Section {
    /// A test result section.
    TestResult(Test),
}

impl Section {
    /// Returns a refernece to a [`Some(TestResult)`] if the [`Section`] is of
    /// type [`Section::TestResult`]. Else, returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::Section;
    /// use bitbelay_report::section::test::Builder;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
    ///
    /// let result = Builder::default()
    ///     .title("Foo")?
    ///     .description("Bar")?
    ///     .push_module(Module::new(Result::Inconclusive, "Baz", None, None))
    ///     .try_build()?;
    ///
    /// let section = Section::TestResult(result);
    /// assert!(matches!(section.as_test_result(), Some(_)));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_test_result(&self) -> Option<&Test> {
        match self {
            Section::TestResult(result) => Some(result),
        }
    }
}
