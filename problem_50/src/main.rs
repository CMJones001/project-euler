use ahash::AHashSet;
use itertools::Itertools;
use std::time::Instant;
use common::get_primes_up_to;

fn main() {
    let limit = 1_000_000;
    let start = Instant::now();
    let (prime_sum, _count) = scan_primes(limit);
    let elapsed = start.elapsed();

    let expected = 997651;

    if prime_sum != expected {
        println!("Expected: {} != {}", expected, prime_sum);
        panic!("Wrong answer");
    } else {
        println!("Last prime: {}", prime_sum);
    }

    println!("Elapsed: {:.2?}", elapsed);
}

fn scan_primes(limit: u64) -> (u64, u64) {
    let primes = get_primes_up_to(limit as usize);
    let n_primes = primes.len();
    let prime_set = primes.iter().copied().collect::<AHashSet<_>>();

    let start: usize = 2;
    (start..n_primes)
        .map(|i| get_series_sum(&primes[i..], limit, &prime_set))
        .max_by_key(|&(_, count)| count)
        .unwrap()
}


/// For the given list of primes, find the sum of the longest series of consecutive primes that
/// sum to a prime less than the limit, starting with the first element.
///
/// Returns the last valid prime sum and number of values in the sequence.
///
/// Requires a prime set for fast lookup (1000x) improvement. The AHashSet is even faster than a
/// Binary search (Vec::binary_search_by_key) for this purpose.
fn get_series_sum(primes: &[u64], limit: u64, prime_set: &AHashSet<u64>) -> (u64, u64) {
    use itertools::FoldWhile::{Continue, Done};

    // Iterate over the primes, keeping track of the last prime that was a sum of consecutive primes
    // and the number of consecutive primes that were summed. We quit when the sum exceeds the limit.
    let (_, _, count, last_prime) = primes
        .iter()
        .fold_while(
            (0, 0, 0, 0),
            |(count, acc_sum, last_prime_count, last_prime), &p| {
                let new_sum = acc_sum + p;
                let new_count = count + 1;

                // If the new sum is prime, update the last prime and the count
                let (last_prime, last_prime_count) = if prime_set.contains(&new_sum) {
                    (new_sum, new_count)
                } else {
                    (last_prime, last_prime_count)
                };

                // If the new sum exceeds the limit, we're done
                if new_sum > limit {
                    Done((count, acc_sum, last_prime_count, last_prime))
                } else {
                    Continue((new_count, new_sum, last_prime_count, last_prime))
                }
            },
        )
        .into_inner();

    (last_prime, count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_100() {
        let primes = get_primes_up_to(100)
            .into_iter()
            .skip_while(|&p| p < 2)
            .collect_vec();
        println!("{:?}", primes);
        let prime_set = primes.iter().copied().collect::<AHashSet<_>>();

        let (last_prime, count) = get_series_sum(&primes, 100, &prime_set);
        assert_eq!(count, 6);
        assert_eq!(last_prime, 41);
    }

    #[test]
    fn scan_1000() {
        let (last_prime, count) = scan_primes(1000);
        assert_eq!(count, 21);
        assert_eq!(last_prime, 953);
    }
}
