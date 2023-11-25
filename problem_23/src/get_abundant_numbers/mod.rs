pub mod brute_force;
pub mod multiplicative_approach;

use crate::proper_divisors::calculate_proper_divisors_doubled;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SumType {
    Abundant,
    Perfect,
    Deficient,
}

pub fn get_sum_type(num: u64) -> SumType {
    // We might make this more efficient by breaking early if the sum is greater than num?
    let divisor_sum = calculate_proper_divisors_doubled(num).iter().sum::<u64>();

    if num == divisor_sum {
        SumType::Perfect
    } else if divisor_sum > num {
        SumType::Abundant
    } else {
        SumType::Deficient
    }
}

#[cfg(test)]
mod test {
    use super::*;
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

    #[test]
    fn test_deficient_numbers() {
        let nums = [1, 16];

        for num in nums {
            assert_eq!(
                SumType::Deficient,
                get_sum_type(num),
                "{num} should be deficient"
            );
        }
    }
}
