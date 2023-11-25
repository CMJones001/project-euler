use std::collections::HashSet;
use std::iter::once;

#[allow(dead_code)]
pub fn calculate_proper_divisors(num: u64) -> impl Iterator<Item = u64> {
    (1..num).filter(move |v| num.rem_euclid(*v) == 0)
}

/// Calculate the proper divisors of a number using the square root method
///
/// We can calculate the proper divisors of a number by iterating up to the square root of the number
/// and checking if the number is divisible by the current value. We then add the reciprocal of the
/// divisor to the set of divisors.
#[allow(dead_code)]
pub fn calculate_proper_divisors_sqrt(num: u64) -> HashSet<u64> {
    // 1 is a special case, as it is not a proper divisor of itself
    if num == 1 {
        return HashSet::new();
    }

    let sqrt = (num as f64).sqrt() as u64;
    let divisors = (1..=sqrt).filter(move |v| num.rem_euclid(*v) == 0);

    let reciprocal_divisors = divisors.clone().filter(|&v| v != 1).map(move |v| num / v);

    divisors.chain(reciprocal_divisors).collect()
}

/// Calculate the proper divisors of a number using the square root method
///
/// We can calculate the proper divisors of a number by iterating up to the square root of the number
/// and checking if the number is divisible by the current value. We then add the reciprocal of the
/// divisor to the set of divisors.
pub fn calculate_proper_divisors_doubled(num: u64) -> HashSet<u64> {
    // 1 is a special case, as it is not a proper divisor of itself
    // We could make this faster by raising an error on 1, and instead returning
    // the iterator...
    if num == 1 {
        return HashSet::new();
    }

    let sqrt = (num as f64).sqrt() as u64;

    // Calculate the divisors (and reciprocal divisors) up to the square root
    // 1 is added, but not the number itself

    // HashSet is used to remove duplicates on square numbers

    (2..=sqrt)
        .filter_map(move |v| {
            let rem = num % v;
            if rem == 0 {
                Some((v, num / v))
            } else {
                None
            }
        })
        .flat_map(|(v1, v2)| [v1, v2])
        .chain(once(1))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_calculate_proper_divisors() {
        let num = 28;
        let mut divisors: Vec<_> = calculate_proper_divisors_doubled(num).into_iter().collect();
        let expected_divisors = vec![1, 2, 4, 7, 14];

        divisors.sort();

        assert_eq!(divisors, expected_divisors)
    }

    #[test]
    fn test_calculate_proper_divisors_sqrt() {
        let num = 28;

        let mut divisors: Vec<_> = calculate_proper_divisors_sqrt(num)
            .into_iter()
            .collect_vec();
        divisors.sort();

        let expected_divisors = vec![1, 2, 4, 7, 14];

        assert_eq!(divisors, expected_divisors)
    }

    #[test]
    fn test_calculate_proper_divisors_square() {
        let num = 16;

        let mut divisors: Vec<_> = calculate_proper_divisors_doubled(num)
            .into_iter()
            .collect_vec();
        divisors.sort();

        let expected_divisors = vec![1, 2, 4, 8];

        assert_eq!(divisors, expected_divisors)
    }

    #[test]
    fn test_calc_proper_divisors_sqrt_equiv() {
        for num in 1..100 {
            let divisors: u64 = calculate_proper_divisors(num).sum();
            let sqrt_divisors: u64 = calculate_proper_divisors_doubled(num).iter().sum();

            assert_eq!(divisors, sqrt_divisors, "{num} should be equal")
        }
    }
}
