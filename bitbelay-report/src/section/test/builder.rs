//! A builder for a [`Test`];

use nonempty::NonEmpty;

use crate::section::test::Module;
use crate::section::Test;

/// An error when a required field is missing.
#[derive(Debug)]
pub enum MissingError {
    /// No title was provided to the [`Builder`].
    Title,

    /// No description was provided to the [`Builder`].
    Description,

    /// No modules were provided to the [`Builder`].
    Modules,
}

impl std::fmt::Display for MissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingError::Title => write!(f, "title"),
            MissingError::Description => write!(f, "description"),
            MissingError::Modules => write!(f, "modules"),
        }
    }
}

impl std::error::Error for MissingError {}

/// An error when multiple values are provided for a singular field.
#[derive(Debug)]
pub enum MultipleError {
    /// Multiple titles were provided to the [`Builder`].
    Title,

    /// Multiple descriptions were provided to the [`Builder`].
    Description,
}

impl std::fmt::Display for MultipleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultipleError::Title => write!(f, "title"),
            MultipleError::Description => write!(f, "description"),
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

/// A builder for a [`Test`].
#[derive(Debug, Default)]
pub struct Builder {
    /// The title.
    title: Option<String>,

    /// The descriptions.
    description: Option<String>,

    /// The modules of the test.
    modules: Option<NonEmpty<Module>>,
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
    /// assert_eq!(result.title(), "Foo");
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

    /// Sets the description for the [`Builder`].
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
    /// assert_eq!(result.description(), "Bar");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Result<Self> {
        let description = description.into();

        if self.description.is_some() {
            return Err(Error::Multiple(MultipleError::Description));
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Pushes a [`Module`] into the [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    ///
    /// let module = Module::new(Result::Inconclusive, "Baz", None, None);
    /// let result = test::Builder::default()
    ///     .title("Foo")?
    ///     .description("Bar")?
    ///     .push_module(module.clone())
    ///     .try_build()?;
    ///
    /// assert_eq!(result.modules().len(), 1);
    /// assert_eq!(result.modules().first(), &module);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn push_module(mut self, module: Module) -> Self {
        let modules = match self.modules {
            Some(mut modules) => {
                modules.push(module);
                modules
            }
            None => NonEmpty::new(module),
        };

        self.modules = Some(modules);
        self
    }

    /// Consumes `self` and attempts to build a [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    ///
    /// let module = Module::new(Result::Inconclusive, "Baz", None, None);
    /// let result = test::Builder::default()
    ///     .title("Foo")?
    ///     .description("Bar")?
    ///     .push_module(module.clone())
    ///     .try_build()?;
    ///
    /// assert_eq!(result.title(), "Foo");
    /// assert_eq!(result.description(), "Bar");
    /// assert_eq!(result.modules().len(), 1);
    /// assert_eq!(result.modules().first(), &module);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_build(self) -> Result<Test> {
        let title = self.title.ok_or(Error::Missing(MissingError::Title))?;

        let description = self
            .description
            .ok_or(Error::Missing(MissingError::Description))?;

        let modules = self.modules.ok_or(Error::Missing(MissingError::Modules))?;

        Ok(Test {
            title,
            description,
            modules,
        })
    }
}
