// Problem 14: Longest Collatz sequence
//
// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even) n → 3n + 1 (n is odd)
//
// Which starting number under one million produces the longest chain?

use std::collections::HashMap;
use std::time::Instant;
use rayon::prelude::*;
use ahash::AHashMap;

fn main() {

    let expected = 837799;
    let range_end = 100_000_000;

    let start_time = Instant::now();
    let (max_steps, max_starting_value) = lookup(range_end);
    let elapsed_time = start_time.elapsed();

    println!("Serial version:");
    println!("Max starting value: {}", max_starting_value);
    println!("Max steps: {}", max_steps);
    println!("Elapsed time: {:?}", elapsed_time);

    if max_starting_value != expected && range_end == 1_000_000 {
        println!("Error: Expected: {}", expected);
    }
    println!();

    let start_time = Instant::now();
    let (max_steps, max_starting_value) = par_lookup(range_end);
    let elapsed_time = start_time.elapsed();

    println!("Parallel version:");
    println!("Max starting value: {}", max_starting_value);
    println!("Max steps: {}", max_steps);
    println!("Elapsed time: {:?}", elapsed_time);

    if max_starting_value != expected && range_end == 1_000_000 {
        println!("Error: Expected: {}", expected);
    }
    println!();

    println!("Serial version (uncached):");
    let start_time = Instant::now();
    let (max_steps, starting_value) = lookup_uncached(range_end);
    let elapsed_time = start_time.elapsed();

    println!("Max steps: {}", max_steps);
    println!("Elapsed time: {:?}", elapsed_time);
    println!();

    println!("Parallel version (uncached):");
    let start_time = Instant::now();
    let (max_steps, starting_value) = par_lookup_uncached(range_end);
    let elapsed_time = start_time.elapsed();

    println!("Max steps: {}", max_steps);
    println!("Elapsed time: {:?}", elapsed_time);
}

/// Get the starting value that produces the longest chain
/// and the number of steps it took to get to 1
fn lookup(range_end: u64) -> (u64, u64) {
    let mut step_dict: AHashMap<u64, u64> = AHashMap::new();
    step_dict.insert(1, 1);

    let mut max_steps = 0;
    let mut max_starting_value = 1;

    for starting_value in (1..=range_end).rev() {
        let steps = add_starting_digits(starting_value, &mut step_dict);
        if steps > max_steps {
            max_steps = steps;
            max_starting_value = starting_value;
        }
    }

    (max_steps, max_starting_value)
}

/// Parallel version of the lookup function
/// This is somewhat faster than the serial version for starting values > 1_000_000
fn par_lookup(range_end: u64) -> (u64, u64) {
    fn starting_dict() -> AHashMap<u64, u64> {
        let mut dict = AHashMap::new();
        dict.insert(1, 1);
        dict
    }

    let (steps, starting_value, _) = (1..=range_end)
        .into_par_iter()
        .fold(|| (1, 1, starting_dict()) ,
          |(max_steps, max_starting_value, mut step_dict), starting_value|{
            let steps = add_starting_digits(starting_value, &mut step_dict);

            if steps > max_steps {
                (steps, starting_value, step_dict)
            } else {
                (max_steps, max_starting_value, step_dict)
            }
        })
        .reduce(|| (1, 1, starting_dict()),
          |(max_steps, max_starting_value, mut step_dict), (steps, starting_value, local_dict)|{
            // Results from fold must be reduced into a single result
            if steps > max_steps {
                (steps, starting_value, local_dict)
            } else {
                (max_steps, max_starting_value, step_dict)
            }
        });

    (steps, starting_value)
}

fn par_lookup_uncached(end_range: u64) -> (u64, u64) {
    (1..=end_range)
        .into_par_iter()
        .map(|starting_value| (get_sequence_length_uncached(starting_value), starting_value))
        .max_by_key(|(steps, _starting_value)| *steps)
        .unwrap()
}

fn lookup_uncached(end_range: u64) -> (u64, u64) {
    (1..=end_range)
        .map(|starting_value| (get_sequence_length_uncached(starting_value), starting_value))
        .max_by_key(|(steps, _starting_value)| *steps)
        .unwrap()
}

/// This function will add the starting digits to the dictionary
/// and return the number of steps it took to get to 1
///
/// We divide by 2 until we get an odd number, and look for this number in the dictionary.
/// If we find it, we can just add the steps to the dictionary and return the total steps.
/// If we don't find it, we add the number to the local history and continue.
/// Once we get to 1, we add all the numbers in the local history to the dictionary.
fn add_starting_digits(num: u64, step_dict: &mut AHashMap<u64, u64>) -> u64 {
    if let Some(steps) = step_dict.get(&num) {
        return *steps;
    }

    let mut num = num;
    let mut steps: u64 = 0;

    let mut local_hist: Vec<(u64, u64)> = Vec::new();

    loop {
        // Divide by 2 until we get an odd number
        let power_two = num.trailing_zeros() as u64;
        num >>= power_two;
        steps += power_two;

        if let Some(prev_steps) = step_dict.get_mut(&num) {
            // If we find the number in the dictionary, we can use the steps from the dictionary
            steps += *prev_steps;

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
        if num == 1 {
            return steps;
        }


        num = 3 * num + 1;
        steps += 1;
    }
}

/// It turns out that computers are _very_ fast at counting zeros, left shifts and multiplication.
/// This method is an order of magnitude faster than the cached versions.
fn get_sequence_length_uncached(val: u64)  -> u64 {
    let mut num = val;
    let mut steps: u64 = 0;

    loop {
        // Divide by 2 until we get an odd number
        let power_two = num.trailing_zeros() as u64;
        num >>= power_two;
        steps += power_two;

        steps += 1;
        if num == 1 {
            // We include the final 1 in the count
            return steps;
        }

        num = 3 * num + 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn get_starting_dict() -> AHashMap<u64, u64> {
        let mut dict = AHashMap::new();
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
        let _count = add_starting_digits(starting_val, &mut dict);

        assert_eq!(dict.get(&61), Some(&20));
        assert_eq!(dict.get(&325), Some(&25));
    }
}
