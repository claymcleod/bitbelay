//! Modules within a test section.

use colored::Colorize as _;

/// A module within a [`Test`](super::Test).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    /// The result.
    result: Result,

    /// The name.
    name: String,

    /// The value.
    value: Option<String>,

    /// Any details regarding the output.
    details: Option<String>,
}

impl Module {
    /// Creates a new [`Module`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
    ///
    /// let module = Module::new(Result::Inconclusive, "Baz", None, None);
    /// assert_eq!(module.result(), &Result::Inconclusive);
    /// assert_eq!(module.name(), "Baz");
    /// assert_eq!(module.value(), None);
    /// assert_eq!(module.details(), None);
    /// ```
    pub fn new(
        result: Result,
        name: impl Into<String>,
        value: Option<String>,
        details: Option<String>,
    ) -> Self {
        Self {
            result,
            name: name.into(),
            value,
            details,
        }
    }

    /// Gets the result from a [`Module`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
    ///
    /// let module = Module::new(Result::Inconclusive, "Baz", None, None);
    /// assert_eq!(module.result(), &Result::Inconclusive);
    /// ```
    pub fn result(&self) -> &Result {
        &self.result
    }

    /// Gets the name from a [`Module`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
    ///
    /// let module = Module::new(Result::Inconclusive, "Baz", None, None);
    /// assert_eq!(module.name(), "Baz");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Gets the value from a [`Module`] (if it exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
    ///
    /// let module = Module::new(
    ///     Result::Inconclusive,
    ///     "Baz",
    ///     Some(String::from("Value")),
    ///     None,
    /// );
    /// assert_eq!(module.value(), Some("Value"));
    /// ```
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Gets the details from a [`Module`] (if they exist).
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
    ///
    /// let module = Module::new(Result::Inconclusive, "Baz", None, None);
    /// assert_eq!(module.details(), None);
    ///
    /// let module = Module::new(
    ///     Result::Inconclusive,
    ///     "Baz",
    ///     None,
    ///     Some(String::from("Foo and bar")),
    /// );
    /// assert_eq!(module.details(), Some("Foo and bar"));
    /// ```
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }
}

/// A result of a module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Result {
    /// A passed module.
    Pass,

    /// An inconclusive module.
    Inconclusive,

    /// A failed module.
    Fail,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Result::Pass => write!(f, "{}", "✓".green().bold()),
                Result::Inconclusive => write!(f, "{}", "?".yellow().bold()),
                Result::Fail => write!(f, "{}", "X".red().bold()),
            }
        } else {
            match self {
                Result::Pass => write!(f, "✓"),
                Result::Inconclusive => write!(f, "?"),
                Result::Fail => write!(f, "X"),
            }
        }
    }
}
