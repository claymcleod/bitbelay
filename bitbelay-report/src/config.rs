//! Configuration of a [`Report`](super::Report).

/// Configuration for a [`Report`](super::Report).
#[derive(Debug)]
pub struct Config {
    /// The width.
    width: usize,

    /// Whether to write out descriptions of each test result.
    write_test_result_descriptions: bool,
}

impl Config {
    /// Gets the width of this report.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.width(), 80);
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns whether or not descriptions of test reports will be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.write_test_result_descriptions(), true);
    /// ```
    pub fn write_test_result_descriptions(&self) -> bool {
        self.write_test_result_descriptions
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            write_test_result_descriptions: true,
        }
    }
}
