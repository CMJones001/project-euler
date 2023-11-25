mod multiplicative_approach;

use itertools::Itertools;
use rayon::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

pub use multiplicative_approach::SumType;

fn main() {
    let num_max = 20161;

    println!("Starting naive approach");

    let abundant_numbers = get_abundant_numbers_parallel(num_max);

    println!("Naive approach finished");

    println!("Starting new approach");

    let abundant_numbers_alt = multiplicative_approach::collect_abundant_numbers(num_max);

    println!("New approach finished");

    let values: HashSet<_> = abundant_numbers
        .iter()
        .combinations(2)
        .filter_map(|v| {
            let total = v.into_iter().sum::<u64>();
            if total > 28123 {
                None
            } else {
                Some(total)
            }
        })
        .collect();

    let numbers: HashSet<u64> = (1..28123).collect();
    let numbers = numbers.difference(&values);

    println!("{:?}", numbers.sum::<u64>())
}

pub fn get_abundant_numbers_parallel(max_val: u64) -> Vec<u64> {
    (12..=max_val)
        .into_par_iter()
        .filter(|v| get_sum_type(*v) == SumType::Abundant)
        .collect()
}

fn get_sum_type(num: u64) -> SumType {
    // We might make this more efficient by breaking early if the sum is greater than num?
    let divisor_sum = calculate_proper_divisors_sqrt(num).iter().sum();
    if num == divisor_sum {
        SumType::Perfect
    } else if divisor_sum > num {
        SumType::Abundant
    } else {
        SumType::Deficient
    }
}

fn calculate_proper_divisors(num: u64) -> impl Iterator<Item = u64> {
    (1..num).filter(move |v| num.rem_euclid(*v) == 0)
}

/// Calculate the proper divisors of a number using the square root method
///
/// We can calculate the proper divisors of a number by iterating up to the square root of the number
/// and checking if the number is divisible by the current value. We then add the reciprocal of the
/// divisor to the set of divisors.
fn calculate_proper_divisors_sqrt(num: u64) -> HashSet<u64> {
    // 1 is a special case, as it is not a proper divisor of itself
    if num == 1 {
        return HashSet::new();
    }

    let sqrt = (num as f64).sqrt() as u64;
    let divisors = (1..=sqrt).filter(move |v| num.rem_euclid(*v) == 0);

    let reciprocal_divisors = divisors.clone().filter(|&v| v != 1).map(move |v| num / v);

    divisors.chain(reciprocal_divisors).collect()
}

fn calculate_proper_divisors_sqrt_alt(num: u64) -> HashSet<u64> {
    use std::iter::once;
    // 1 is a special case, as it is not a proper divisor of itself
    if num == 1 {
        return HashSet::new();
    }

    let sqrt = (num as f64).sqrt() as u64;

    // Calculate the divisors (and reciprocal divisors) up to the square root
    // 1 is added, but not the number itself

    (2..=sqrt)
        .filter_map(move |v| {
            let rem = num % v;
            if rem == 0 {
                Some((v, num / v))
            } else {
                None
            }
        })
        .flat_map(|(v1, v2)| once(v1).chain(once(v2)))
        .chain(once(1))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_proper_divisors() {
        let num = 28;

        let divisors: Vec<_> = calculate_proper_divisors(num).collect();

        let expected_divisors = vec![1, 2, 4, 7, 14];

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
    fn test_calc_proper_divisors_sqrt_equiv() {
        for num in 1..100 {
            let divisors: u64 = calculate_proper_divisors(num).sum();
            let sqrt_divisors: u64 = calculate_proper_divisors_sqrt(num).iter().sum();

            assert_eq!(divisors, sqrt_divisors, "{num} should be equal")
        }
    }

    #[test]
    fn test_perfect_numbers() {
        let nums = [28];

        for num in nums {
            assert_eq!(
                SumType::Perfect,
                get_sum_type(num),
                "{num} should be a perfect number"
            );
        }
    }

    #[test]
    fn test_abundant_numbers() {
        let nums = [12];

        for num in nums {
            assert_eq!(
                SumType::Abundant,
                get_sum_type(num),
                "{num} should be abundant"
            );
        }
    }
}
