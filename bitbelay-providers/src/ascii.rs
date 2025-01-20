//! ASCII data providers.

use rand::Rng;
use rand::rngs::ThreadRng;

/// An alphanumeric ASCII data provider.
///
/// Generates random alphanumeric ASCII bytes as unicode characters (typical of
/// how they would be stored in the wild).
#[derive(Clone, Debug)]
pub struct AlphanumericProvider {
    /// The name.
    name: String,

    /// The number of characters to provide per call.
    length: usize,

    /// The current data stored in the provider.
    data: Vec<String>,

    /// A thread-local random generator.
    rng: ThreadRng,
}

impl AlphanumericProvider {
    /// Creates a new ASCII data provider that returns `length` alphanumeric
    /// unicode characters.
    ///
    /// # Examples
    ///
    /// ```
    /// // The trait must also be in scope to access the `provide()` method.
    /// use bitbelay_providers::Provider as _;
    /// use bitbelay_providers::ascii::AlphanumericProvider;
    ///
    /// let mut provider = AlphanumericProvider::new(10);
    /// let data = provider.provide(20);
    /// assert_eq!(data.len(), 20);
    /// assert_eq!(data.first().unwrap().len(), 10);
    /// ```
    pub fn new(length: usize) -> Self {
        Self {
            name: format!("ASCII Alphanumeric ({} characters)", length),
            length,
            data: Vec::default(),
            rng: rand::thread_rng(),
        }
    }
}

impl crate::Provider for AlphanumericProvider {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn provide(&mut self, n: usize) -> Vec<&[u8]> {
        self.data = Vec::with_capacity(n);

        // NOTE: this method goes from bytes to String and back to bytes—why not just
        // stick with the original bytes? In short, though I find this possibility
        // unlikely, it's because the representation of [`String`] _may_ change in the
        // future, and I didn't want to have to come back and change this if that
        // happens. Thus, I made the decision to take the longer route to ensure that
        // the data is _exactly_ how [`String`]s are represented today.
        for _ in 0..n {
            let value = (&mut self.rng)
                .sample_iter(rand::distributions::Alphanumeric)
                .take(self.length)
                .map(char::from)
                .collect::<String>();

            self.data.push(value);
        }

        self.data.iter().map(|x| x.as_bytes()).collect::<Vec<_>>()
    }

    fn bytes_per_input(&mut self) -> usize {
        // NOTE: all alphanumeric ASCII characters fall within the range of a singe byte
        // (`u8`). As such, when the bytes are collated into a UTF-8 [`String`], these
        // characters will only take up 1 byte each. Thus, even though the length of a
        // [`char`] is four bytes, the length of these inputs will always be the length
        // of the String.
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Provider;

    #[test]
    fn it_correctly_calculates_bytes_per_input() {
        let mut provider = AlphanumericProvider::new(10);
        // SAFETY: we provided one input, so the direct index to `0` will always
        // succeed.
        let data = provider.provide(1)[0];
        assert_eq!(data.len(), provider.bytes_per_input());
    }
}
