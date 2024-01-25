//! Reporting facilities for `bitbelay`.

use std::io::Write;

use chrono::DateTime;
use chrono::Local;
use colored::Colorize as _;
use nonempty::NonEmpty;
use textwrap::Options;

mod builder;
mod config;
pub mod section;

pub use builder::Builder;
pub use config::Config;
pub use section::Section;

use crate::section::test::Module;

// NOTE: though it is not statically checked, each of the [`&str`] below should
// all be one character in length. They were declared as [`&str`] instead of
// [`char`] to simplify the code and reduce the number of allocations needed.
// Just keep in mind that all of the math when generating these reports assumes
// these are one character!

/// The character to enclose a title in when writing a title block.
const TITLE_BLOCK_CHAR: &str = "#";

/// The divider section within a section.
const SECTION_DIVIDER_CHAR: &str = "=";
/// The character to use when creating a horizontal rule within a section.
const SECTION_HR_CHAR: &str = "-";
/// The gutter characters (used on the left and right) to frame a section.
const SECTION_VERTICAL_BLOCK_CHAR: &str = "|";

/// A report for a suite of tests.
///
/// The report is comprised of a few elements:
///
/// * The title of the test suite.
/// * The date that the test suite was run.
/// * The sections within the report.
#[derive(Debug)]
pub struct Report {
    /// The title of the test suite.
    title: String,

    /// The date that the test suite was run.
    date: DateTime<Local>,

    /// The sections within the report.
    sections: NonEmpty<Section>,
}

impl Report {
    /// Gets the title from the [`Report`].
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
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    /// Gets the date from the [`Report`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    /// use chrono::Local;
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
    /// assert_eq!(
    ///     Local::now().naive_local().date(),
    ///     report.date().naive_local().date()
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn date(&self) -> DateTime<Local> {
        self.date
    }

    /// Gets the sections from a [`Report`].
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
    pub fn sections(&self) -> &NonEmpty<Section> {
        &self.sections
    }

    /// Writes the report to a [writer](std::io::Write).
    ///
    /// # Examples
    ///
    /// ```
    /// use bitbelay_report::section::test;
    /// use bitbelay_report::section::test::module::Result;
    /// use bitbelay_report::section::test::Module;
    /// use bitbelay_report::Builder;
    /// use bitbelay_report::Config;
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
    /// let mut buffer = Vec::new();
    /// report.write_to(&mut buffer, &Config::default())?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn write_to<W: Write>(&self, writer: &mut W, config: &Config) -> std::io::Result<()> {
        write_title_block(writer, &format!("{} Test Suite", &self.title), config)?;
        write_centered_line(writer, &format!("Date: {:#?}", self.date), config)?;

        for section in &self.sections {
            writeln!(writer)?;
            match section {
                Section::TestResult(section) => write_test_result(writer, section, config)?,
            }
        }

        Ok(())
    }
}

//=============//
// Foundations //
//=============//

/// Gets the length of the visible string that will be printed to the terminal.
///
/// In other words, it does not count terminal control sequences that modify the
/// style or color of the text.
fn visible_length(s: &str) -> usize {
    let mut length = 0;
    let mut in_escape_sequence = false;

    for char in s.chars() {
        match char {
            '\u{1b}' => in_escape_sequence = true,
            'a'..='z' | 'A'..='Z' if in_escape_sequence => in_escape_sequence = false,
            _ if !in_escape_sequence => length += 1,
            _ => {}
        }
    }

    length
}

/// Gets the required padding to center a line within a report.
fn get_padding(element_width: usize, config: &Config) -> usize {
    if element_width > config.width() {
        0
    } else {
        (config.width() - element_width) / 2
    }
}

//=========//
// General //
//=========//

/// Writes a centered line within a report of a given configuration.
fn write_centered_line<W: Write>(
    writer: &mut W,
    line: &str,
    config: &Config,
) -> std::io::Result<()> {
    let padding = get_padding(visible_length(line), config);

    writeln!(
        writer,
        "{:padding$}{}{:padding$}",
        "",
        line,
        "",
        padding = padding
    )?;

    Ok(())
}

//=======//
// Title //
//=======//

/// Prints the title block for a report.
fn write_title_block<W: Write>(writer: &mut W, line: &str, config: &Config) -> std::io::Result<()> {
    let element_width = visible_length(line) + 4; // Two spaces and two block chars.

    if config.width() < element_width {
        panic!(
            "total width ({}) too small to print block ({})",
            config.width(),
            element_width
        );
    }

    write_centered_line(writer, &TITLE_BLOCK_CHAR.repeat(element_width), config)?;
    write_centered_line(
        writer,
        &format!("{} {} {}", &TITLE_BLOCK_CHAR, line, &TITLE_BLOCK_CHAR),
        config,
    )?;
    write_centered_line(writer, &TITLE_BLOCK_CHAR.repeat(element_width), config)?;
    writeln!(writer)
}

//==========//
// Sections //
//==========//

/// Writes the start of a new section within the report.
fn write_section_start<W: Write>(writer: &mut W, config: &Config) -> std::io::Result<()> {
    writeln!(
        writer,
        "/{}\\",
        SECTION_DIVIDER_CHAR.repeat(config.width() - 2)
    ) // The two slashes.
}

/// Writes the end of a section within the report.
fn write_section_end<W: Write>(writer: &mut W, config: &Config) -> std::io::Result<()> {
    writeln!(
        writer,
        "\\{}/",
        SECTION_DIVIDER_CHAR.repeat(config.width() - 2)
    ) // The two slashes.
}

/// Writes a line within a section of the report.
fn write_section_line<W: Write>(
    writer: &mut W,
    line: &str,
    config: &Config,
) -> std::io::Result<()> {
    let line_len = visible_length(line) + 4; // Two spaces and two description block chars.

    if config.width() < line_len {
        panic!(
            "description line length ({}) is too long to print within total width ({})",
            line_len,
            config.width()
        )
    }

    let padding = config.width() - line_len;

    writeln!(
        writer,
        "{} {} {:padding$}{}",
        SECTION_VERTICAL_BLOCK_CHAR,
        line,
        "",
        SECTION_VERTICAL_BLOCK_CHAR,
        padding = padding
    )
}

/// Writes lines within a section of the report while wrapping lines as
/// necessary.
///
/// # Notes
///
/// * Unforunately, there is no good way that I could think of to not count
///   terminal control sequences here: `textwrap` does not support ignoring
///   them, and they must be included in the text passed to `textwrap` to be
///   printed. As such, their length just counts when wrapping lines here,
///   potentially leading to lines that are wrapped "too early" (because
///   `textwrap` thinks they are longer than they actually are when displayed).
fn write_section_wrapped_lines<W: Write>(
    writer: &mut W,
    lines: &str,
    config: &Config,
) -> std::io::Result<()> {
    let max_line_length = config.width() - 4; // Two spaces and to description block chars.

    for line in textwrap::wrap(lines, Options::new(max_line_length)) {
        write_section_line(writer, &line, config)?;
    }

    Ok(())
}

//===================//
// Parts of Sections //
//===================//

/// Writes a section title.
fn write_section_title<W: Write>(
    writer: &mut W,
    title: &str,
    config: &Config,
) -> std::io::Result<()> {
    let title = format!("{} {}", "#".bold(), title.underline().bold());
    write_section_line(writer, &title, config)
}

/// Writes a horizontal rule within a section.
fn write_section_hr<W: Write>(writer: &mut W, config: &Config) -> std::io::Result<()> {
    writeln!(
        writer,
        "{}{}{}",
        SECTION_VERTICAL_BLOCK_CHAR,
        SECTION_HR_CHAR.repeat(config.width() - 2),
        SECTION_VERTICAL_BLOCK_CHAR
    )
}

/// Writes a module within a section.
fn write_section_module<W: Write>(
    writer: &mut W,
    module: &Module,
    config: &Config,
) -> std::io::Result<()> {
    let mut summary_line = format!("[{:#}] {}", module.result(), module.name().bold());

    if let Some(value) = module.value() {
        summary_line.push_str(&format!(" => {}", value));
    }

    write_section_line(writer, &summary_line, config)?;

    if let Some(details) = module.details() {
        write_section_line(writer, "", config)?;
        write_section_wrapped_lines(writer, details, config)?;
    }

    Ok(())
}

/// Writes a full test result section.
fn write_test_result<W: Write>(
    writer: &mut W,
    section: &section::Test,
    config: &Config,
) -> std::io::Result<()> {
    // Header.
    write_section_start(writer, config)?;
    write_section_line(writer, "", config)?;
    write_section_title(writer, section.title(), config)?;
    write_section_line(writer, "", config)?;
    write_section_hr(writer, config)?;

    // Description.
    if config.write_test_result_descriptions() {
        write_section_line(writer, "", config)?;
        write_section_wrapped_lines(writer, section.description(), config)?;
        write_section_line(writer, "", config)?;
        write_section_hr(writer, config)?;
    }

    // Modules.
    write_section_line(writer, "", config)?;
    for module in section.modules() {
        write_section_module(writer, module, config)?;
    }

    // Footer.
    write_section_line(writer, "", config)?;
    write_section_end(writer, config)?;

    Ok(())
}
