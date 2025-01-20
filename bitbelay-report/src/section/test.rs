//! A section describing a test result.

use nonempty::NonEmpty;

mod builder;
pub mod module;

pub use builder::Builder;
pub use module::Module;

/// A section of a [`Report`](crate::Report) describing a test that was
/// conducted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Test {
    /// The test title.
    title: String,

    /// The test description.
    description: String,

    /// The modules of the test.
    modules: NonEmpty<Module>,
}

impl Test {
    /// Gets the title from the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::Builder;
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
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
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    /// Gets the description from the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::Builder;
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
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
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    /// Gets the [`Module`]s from the [`Test`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::Builder;
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::section::test::module::Result;
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
    pub fn modules(&self) -> &NonEmpty<Module> {
        &self.modules
    }
}
