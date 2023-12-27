<p align="center">
  <h1 align="center">
    bitbelay
  </h1>

  <p align="center">
    <a href="https://github.com/claymcleod/bitbelay/actions/workflows/ci.yml" target="_blank">
      <img alt="CI: Status" src="https://github.com/claymcleod/bitbelay/actions/workflows/ci.yml/badge.svg" />
    </a>
    <a href="https://github.com/claymcleod/bitbelay/blob/master/LICENSE-APACHE" target="_blank">
      <img alt="License: Apache 2.0" src="https://img.shields.io/badge/license-Apache 2.0-blue.svg" />
    </a>
    <a href="https://github.com/claymcleod/bitbelay/blob/master/LICENSE-MIT" target="_blank">
      <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-blue.svg" />
    </a>
  </p>


  <p align="center">
    A performance evaluation harness for non-cryptographic hash functions.
    <br />
    <br />
    <a href="https://github.com/claymcleod/bitbelay/issues/new?assignees=&title=Descriptive%20Title&labels=enhancement">Request Feature</a>
    ·
    <a href="https://github.com/claymcleod/bitbelay/issues/new?assignees=&title=Descriptive%20Title&labels=bug">Report Bug</a>
    ·
    ⭐ Consider starring the repo! ⭐
    <br />
    <br />
  </p>
</p>

Bitbelay is a framework for testing the performance and desirable characteristics of
non-cryptographic hashing functions. The project was given this name because (a) it is
concerned with evaluating the characteristics of bits output from hash functions and (b)
["belaying"](https://en.wikipedia.org/wiki/Belaying) evokes imagery of a (test) harness.

Bitbelay is designed somewhat differently than other popular hash testing frameworks
[[1][smhasher]]. For example, it does not a ship a single binary that is used to
benchmark performance across multiple hash functions. Instead, it (a) is comprised of a
family of crates that provide high-quality facilities for testing hash functions and (b)
enables hash developers to easily wrap their hash functions in a command line tool for
performance testing.

Command line tools are generally written and published containing the facilities for
characterizing an individual hash function. When publishing these on
[crates.io](https://crates.io/) or elsewhere, the convention is to name the crate and/or
associated command line tool as `bitbelay-[HASHNAME]` (e.g., `bitbelay-ahash` for
`ahash`) so that it can be easily identified.

## 🎨 Features

* **Advanced hash characterization.** Bitbelay's primary goal is to provide facilities
  for characterizing the performance and quality of non-cryptographic hash functions. As
  such, it contains an extensive set of tests organized into a collection of
  purpose-built test suites.
* **Multiple data providers.** Bitbelay includes a range of data providers to facilitate
  the assessment of hash functions against a variety of input data types. This diversity
  allows for a more comprehensive understanding of hash function performance across
  different scenarios. Further, custom data providers can be seamlessly integrated into
  the framework.
* **Drop-in testing for hash function development.** Bitbelay aims to ease the process
  of developing hash functions—especially in Rust! To accomplish this, it includes
  facilities to easily wrap a hash function as a command-line tool within which a
  battery of tests can be employed.

## 📚 Getting Started

You can add `bitbelay` as a dependency via the Github repository. 

```bash
cargo add bitbelay
```

Next, you can use the `bitbelay::cli::wrapper()` function to quickly wrap a hash
function of interest and produce a command-line tool for evaluating it. For this
example, we simply pull in Rust's d

```rust
use std::hash::RandomState;

pub fn main() -> anyhow::Result<()> {
    bitbelay::cli::wrapper(RandomState::default())
}
```

## Examples

You can also take a look at the
[examples](https://github.com/claymcleod/bitbelay/tree/main/bitbelay/examples) to
get a sense of the various ways you can use the crate.


## 🖥️ Development

To bootstrap a development environment, please use the following commands.

```bash
# Clone the repository
git clone git@github.com:claymcleod/bitbelay.git
cd bitbelay 

# Build the crate in release mode
cargo build --release

# List out the examples
cargo run --release --example
```

## 🚧️ Tests

Before submitting any pull requests, please make sure the code passes the following
checks.

```bash
# Run the project's tests.
cargo test --all-features

# Ensure the project doesn't have any linting warnings.
cargo clippy --all-features

# Ensure the project passes `cargo fmt`.
cargo fmt --check

# Ensure the docs build successfully.
cargo doc
```

## Minumum Supported Rust Version (MSRV)

As bitbelay is pre-1.0, no MSRV is yet asserted.

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to check [issues
page](https://github.com/claymcleod/bitbelay/issues).

## 📝 License

This project is licensed as either [Apache 2.0][license-apache] or [MIT][license-mit] at
your discretion.

Copyright © 2024-Present [Clay McLeod](https://github.com/claymcleod).

[license-apache]: https://github.com/claymcleod/bitbelay/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/claymcleod/bitbelay/blob/master/LICENSE-MIT

[smhasher]: https://github.com/rurban/smhasher