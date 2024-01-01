use itertools::Itertools;
use std::time::Instant;
use common::get_primes_up_to;

fn main() {
    let limit = 1_000_000;
    let start = Instant::now();
    let (prime_sum, count) = scan_primes(limit);
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

    let start: usize = 2;
    (start..n_primes).map(|i| get_series_sum(&primes[i..], limit)).max_by_key(|&(_, count)| count).unwrap()
}

fn get_series_sum(primes: &[u64], limit: u64) -> (u64, u64) {
    use itertools::FoldWhile::{Continue, Done};

    let (_, _, last_prime, count) = primes.iter().fold_while((0, 0, 1, 0), |(count, acc_sum, last_prime, last_prime_count), &p| {
        let new_sum = acc_sum + p;

        let (last_prime, last_prime_count) = if primes.contains(&new_sum) {
            (new_sum, count + 1)
        } else {
            (last_prime, last_prime_count)
        };

        if new_sum > limit {
            Done((count, acc_sum, last_prime, last_prime_count))
        } else {
            Continue((count + 1, new_sum, last_prime, last_prime_count))
        }

    }).into_inner();

    (last_prime, count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_100() {
        let primes = get_primes_up_to(100).into_iter().skip_while(|&p| p < 2).collect_vec();
        println!("{:?}", primes);

        let (last_prime, count) = get_series_sum(&primes,  100);
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