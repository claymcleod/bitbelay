//! Numeric data providers.

use rand::Rng as _;
use rand::rngs::ThreadRng;

/// A `u64` data provider.
///
///
/// # Notes
///
/// * `u64` are always stored in an **little endian** fashion to avoid any
///   variances due to platform storage conventions.
#[derive(Debug, Default)]
pub struct Unsigned64BitProvider {
    /// The name.
    name: String,

    /// The number of `u64`s to provide per call.
    length: usize,

    /// The current data stored in the provider.
    data: Vec<Vec<u8>>,

    /// A thread-local random generator.
    rng: ThreadRng,
}

impl Unsigned64BitProvider {
    /// Creates a new `u64` data provider that returns `length` 64-bit unsigned
    /// integers (stored in a little endian representation).
    ///
    /// # Examples
    ///
    /// ```
    /// // The trait must also be in scope to access the `provide()` method.
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::numeric::Unsigned64BitProvider;
    ///
    /// let mut provider = Unsigned64BitProvider::new(10);
    ///
    /// let data = provider.provide(20);
    /// assert_eq!(data.len(), 20);
    /// // Note that each u64 is eight (8) bytes, so we can expect a length
    /// // of `10 * 8 = 80`.
    /// assert_eq!(data.first().unwrap().len(), 80);
    /// ```
    pub fn new(length: usize) -> Self {
        Self {
            name: format!("Unsigned 64-bit integers (n={})", length),
            length,
            data: Vec::with_capacity(length),
            rng: rand::thread_rng(),
        }
    }
}

impl crate::Provider for Unsigned64BitProvider {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn provide(&mut self, n: usize) -> Vec<&[u8]> {
        self.data.clear();

        for _ in 0..n {
            let mut buffer = Vec::with_capacity(self.length);
            for _ in 0..self.length {
                let random_value = self.rng.gen::<u64>();
                buffer.extend_from_slice(&random_value.to_le_bytes());
            }
            self.data.push(buffer);
        }

        self.data.iter().map(|x| x.as_slice()).collect::<Vec<_>>()
    }

    fn bytes_per_input(&mut self) -> usize {
        std::mem::size_of::<u64>() * self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Provider;

    #[test]
    fn it_correctly_calculates_bytes_per_input() {
        let mut provider = Unsigned64BitProvider::new(10);
        // SAFETY: we provided one input, so the direct index to `0` will always
        // succeed.
        let data = provider.provide(1)[0];
        assert_eq!(data.len(), provider.bytes_per_input());
    }
}
