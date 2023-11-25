use crate::get_abundant_numbers::{get_sum_type, SumType};

use rayon::prelude::*;

/// Brute force approach to finding abundant numbers
///
/// This function will find all abundant numbers up to and including `max_val`
/// As this approach can be parallelized, with enough cores, this approach can be
/// faster than the multiplicative approach.
pub fn collect_abundant_numbers_parallel(max_val: u64) -> Vec<u64> {
    (12..=max_val)
        .into_par_iter()
        .filter(|v| get_sum_type(*v) == SumType::Abundant)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect_abundant_numbers() {
        let abundant_numbers = collect_abundant_numbers_parallel(60);
        let abundant_numbers_expected = [12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60];

        assert_eq!(abundant_numbers, abundant_numbers_expected)
    }
}
