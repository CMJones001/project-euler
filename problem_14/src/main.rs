// Problem 14: Longest Collatz sequence
//
// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even) n → 3n + 1 (n is odd)
//
// Which starting number under one million produces the longest chain?

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut step_dict: HashMap<u64, u64> = HashMap::new();
    step_dict.insert(1, 1);

    let range_end = 1_000_000;
    let mut max_steps = 0;
    let mut max_starting_value = 1;

    let starting_time = Instant::now();
    for starting_value in (1..=range_end).rev() {
        let steps = add_starting_digits(starting_value, &mut step_dict);
        if steps > max_steps {
            max_steps = steps;
            max_starting_value = starting_value;
        }
    }
    let elapsed_time = starting_time.elapsed();

    println!("Max starting value: {}", max_starting_value);
    println!("Max steps: {}", max_steps);
    println!("Elapsed time: {:?}", elapsed_time);
}

/// This function will add the starting digits to the dictionary
/// and return the number of steps it took to get to 1
///
/// We divide by 2 until we get an odd number, and look for this number in the dictionary.
/// If we find it, we can just add the steps to the dictionary and return the total steps.
/// If we don't find it, we add the number to the local history and continue.
/// Once we get to 1, we add all the numbers in the local history to the dictionary.
fn add_starting_digits(num: u64, step_dict: &mut HashMap<u64, u64>) -> u64 {
    if let Some(steps) = step_dict.get(&num) {
        return *steps;
    }

    let mut num = num;
    let mut steps: u64 = 0;

    let mut local_hist: Vec<(u64, u64)> = Vec::new();

    loop {
        // Divide by 2 until we get an odd number
        let power_two = num.trailing_zeros() as u64;
        if power_two > 0 {
            num >>= power_two;
            steps += power_two;
        }

        if step_dict.contains_key(&num) {
            // If we find the number in the dictionary, we can use the steps from the dictionary
            steps += step_dict.get(&num).unwrap();

            // Now we need to add all the numbers in the local history to the dictionary
            for (num, hist_steps) in local_hist {
                step_dict.insert(num, steps - hist_steps);
            }
            return steps;
        } else {
            // If we don't find the odd number in the dictionary, we need to add it to the local history
            // so that we can add it to the dictionary later
            local_hist.push((num, steps));
        }

        num = 3 * num + 1;
        steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn get_starting_dict() -> HashMap<u64, u64> {
        let mut dict = HashMap::new();
        dict.insert(1, 1);
        dict
    }

    #[test_case(8, 4)]
    #[test_case(16, 5)]
    #[test_case(5, 6)]
    #[test_case(20, 8)]
    #[test_case(13, 10)]
    #[test_case(27, 112)]
    fn test_add_starting_digits(starting_value: u64, expected_steps: u64) {
        let mut dict = get_starting_dict();
        let count = add_starting_digits(starting_value, &mut dict);

        assert_eq!(count, expected_steps);
    }

    #[test]
    fn test_examine_history_dict() {
        let starting_val = 13;
        let mut dict = get_starting_dict();
        let count = add_starting_digits(starting_val, &mut dict);

        assert_eq!(count, 10);
        println!("{:?}", dict);

        assert_eq!(dict.get(&5), Some(&6));
        assert_eq!(dict.get(&13), Some(&10));
        assert_eq!(dict.get(&1), Some(&1));
    }

    #[test]
    fn test_examine_history_dict_23() {
        let starting_val = 23;
        let mut dict = get_starting_dict();
        let count = add_starting_digits(starting_val, &mut dict);

        assert_eq!(count, 16);
        println!("{:?}", dict);

        assert_eq!(dict.get(&5), Some(&6));
        assert_eq!(dict.get(&53), Some(&12));
        assert_eq!(dict.get(&35), Some(&14));
        assert_eq!(dict.get(&23), Some(&16));

        // This value should chain onto 23
        let starting_val = 325;
        let count = add_starting_digits(starting_val, &mut dict);

        assert_eq!(dict.get(&61), Some(&20));
        assert_eq!(dict.get(&325), Some(&25));
    }
}
