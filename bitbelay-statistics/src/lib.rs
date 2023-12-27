//! Statistical tests used within `bitbelay`.
//!
//! # Types of Tests
//!
//! * Tests related to the [Chi-squared distribution] are located in the
//!   `chi_squared` module ([link](chi_squared)).
//! * Tests related to the correlation, such as [Pearson] and [Spearman]
//!   correlation, are located in the `correlation` module
//!   ([link](correlation)).
//!
//! [Chi-squared distribution]: https://en.wikipedia.org/wiki/Chi-squared_distribution
//! [Pearson]: https://en.wikipedia.org/wiki/Pearson_correlation_coefficient
//! [Spearman]: https://en.wikipedia.org/wiki/Spearman%27s_rank_correlation_coefficient

use std::collections::BTreeMap;

pub mod chi_squared;
pub mod correlation;

/// Ranks the inputs according to their [sort order](std::cmp::Ord`).
fn rank<T: Clone + Ord>(data: &[T]) -> Vec<usize> {
    let mut sorted = data.to_vec();
    sorted.sort();

    let mut ranks = BTreeMap::new();
    let mut current_rank = 1usize;

    for value in sorted {
        ranks.entry(value.clone()).or_insert_with(|| {
            let rank = current_rank;
            current_rank += 1;
            rank
        });
    }

    // SAFETY: we just went through every value in `data` above, so we know every
    // element now exists and will be retrieved within `ranks`. Thus, this will
    // always unwrap.
    data.iter()
        .map(|v| *ranks.get(v).unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::rank;

    #[test]
    fn rank_works_correctly() {
        let input = &[1, 3, 5, 2, 4, 6];
        assert_eq!(rank(input), input);

        let input = &[20, 10, 40, 30];
        assert_eq!(rank(input), &[2, 1, 4, 3]);
    }
}
