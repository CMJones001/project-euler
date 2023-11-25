/// We make use of the fact that the product of an abundant number is also abundant
/// The product of a perfect number is abundant (except the perfect number itself)
/// This allows us to reduce the number of numbers we need to check
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SumType {
    Abundant,
    Perfect,
    Deficient,
}

/// Generate a vector of abundant numbers up to max_val
pub fn collect_abundant_numbers(max_val: u64) -> Vec<u64> {
    classify_numbers(max_val as usize)
        .into_iter()
        .enumerate()
        .filter_map(|(index, s)| {
            if s != SumType::Abundant {
                None
            } else {
                Some(index as u64)
            }
        })
        .collect()
}

/// Generate a vector of SumType for each number up to max_val
///
/// For an index i, the value at i is the SumType for i
/// This is poorly defined for i = 0, so we just give it a value
pub fn classify_numbers(max_val: usize) -> Vec<SumType> {
    let mut numbers: Vec<Option<SumType>> = vec![None; max_val + 1];

    for num in 1..=max_val {
        let selection = numbers[num];

        if selection.is_some() {
            continue;
        }

        let num_type = crate::get_sum_type(num as u64);
        match num_type {
            SumType::Abundant => {
                for x in (num..=max_val).step_by(num) {
                    numbers[x] = Some(SumType::Abundant)
                }
            }
            SumType::Perfect => {
                numbers[num] = Some(SumType::Perfect);
                for x in ((num * 2)..=max_val).step_by(num) {
                    numbers[x] = Some(SumType::Abundant)
                }
            }
            SumType::Deficient => numbers[num] = Some(SumType::Deficient),
        }
    }

    // Convenient to have 0 in the array, so give some arbitrary value
    numbers[0] = Some(SumType::Deficient);

    let numbers: Option<Vec<_>> = numbers.into_iter().collect();
    numbers.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_classify_numbers() {
        let classified = classify_numbers(300);

        let perfect_numbers = [6, 28];

        for p in perfect_numbers {
            assert_eq!(classified[p], SumType::Perfect, "{p} should be perfect")
        }

        let abundant_numbers = [12, 18, 20, 24, 30, 36, 40, 42, 48];
        for a in abundant_numbers {
            assert_eq!(classified[a], SumType::Abundant, "{a} should be abundant")
        }
    }

    #[test]
    fn test_collect_abundant_numbers() {
        let abundant_numbers = collect_abundant_numbers(60);
        let abundant_numbers_expected = [12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60];

        assert_eq!(abundant_numbers, abundant_numbers_expected)
    }
}
