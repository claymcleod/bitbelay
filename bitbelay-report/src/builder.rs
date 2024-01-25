//! Builders for [`Report`]s.

use chrono::Local;
use nonempty::NonEmpty;

use crate::section::Section;
use crate::section::Test;
use crate::Report;

/// An error when a required field is missing.
#[derive(Debug)]
pub enum MissingError {
    /// No title was provided to the [`Builder`].
    Title,

    /// No sections where provided to the [`Builder`].
    Sections,
}

impl std::fmt::Display for MissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingError::Title => write!(f, "title"),
            MissingError::Sections => write!(f, "sections"),
        }
    }
}

impl std::error::Error for MissingError {}

/// An error when multiple values are provided for a singular field.
#[derive(Debug)]
pub enum MultipleError {
    /// Multiple titles were provided to the [`Builder`].
    Title,
}

impl std::fmt::Display for MultipleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultipleError::Title => write!(f, "title"),
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

/// A builder for a [`Report`].
#[derive(Debug, Default)]
pub struct Builder {
    /// The title.
    title: Option<String>,

    /// The sections.
    sections: Option<NonEmpty<Section>>,
}

impl Builder {
    /// Sets the title for the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    ///
    /// let result = test::Builder::default()
    ///     .title("Foo")?
    ///     .description("Bar")?
    ///     .push_module(Module::new(Result::Inconclusive, "Baz", None, None))
    ///     .try_build()?;
    ///
    /// let report = Builder::default()
    ///     .title("Hello, world!")?
    ///     .push_test_result(result)
    ///     .try_build()?;
    ///
    /// assert_eq!(report.title(), "Hello, world!");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Result<Self> {
        let title = title.into();

        if self.title.is_some() {
            return Err(Error::Multiple(MultipleError::Title));
        }

        self.title = Some(title);
        Ok(self)
    }

    /// Pushes a [test result section](Test) into the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    ///
    /// let result = test::Builder::default()
    ///     .title("Foo")?
    ///     .description("Bar")?
    ///     .push_module(Module::new(Result::Inconclusive, "Baz", None, None))
    ///     .try_build()?;
    ///
    /// let report = Builder::default()
    ///     .title("Hello, world!")?
    ///     .push_test_result(result.clone())
    ///     .try_build()?;
    ///
    /// assert_eq!(report.sections().len(), 1);
    /// assert_eq!(report.sections().first().as_test_result().unwrap(), &result);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn push_test_result(mut self, result: Test) -> Self {
        let section = Section::TestResult(result);

        let sections = match self.sections {
            Some(mut sections) => {
                sections.push(section);
                sections
            }
            None => NonEmpty::new(section),
        };

        self.sections = Some(sections);
        self
    }

    /// Consumes `self` and attempts to build a [`Report`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    ///
    /// let result = test::Builder::default()
    ///     .title("Foo")?
    ///     .description("Bar")?
    ///     .push_module(Module::new(Result::Inconclusive, "Baz", None, None))
    ///     .try_build()?;
    ///
    /// let report = Builder::default()
    ///     .title("Hello, world!")?
    ///     .push_test_result(result.clone())
    ///     .try_build()?;
    ///
    /// assert_eq!(report.title(), "Hello, world!");
    /// assert_eq!(report.sections().len(), 1);
    /// assert_eq!(report.sections().first().as_test_result().unwrap(), &result);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_build(self) -> Result<Report> {
        let title = self.title.ok_or(Error::Missing(MissingError::Title))?;

        let sections = self
            .sections
            .ok_or(Error::Missing(MissingError::Sections))?;

        Ok(Report {
            title,
            date: Local::now(),
            sections,
        })
    }
}
