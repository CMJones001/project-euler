use std::collections::{HashSet, VecDeque};
use std::time::Instant;

// Day 47: Distinct primes factors
//
// We are looking for the first four consecutive numbers that have four distinct prime factors each.

fn main() {
    println!("Problem 47");

    let n = 200_000;
    let num_factors = 4;

    let start = Instant::now();
    let answer = get_consecutive_prime_factors(n, num_factors)
        .expect("No consecutive numbers with 4 prime factors for n < {n}");
    let duration = start.elapsed();

    println!("Answer: {}", answer);
    println!("Time taken: {:?}", duration);
}

fn get_consecutive_prime_factors(n: usize, num_factors: usize) -> Option<usize> {
    let primes = get_primes_up_to(n-4);
    let mut previous_factors = VecDeque::with_capacity(num_factors);

    for i in 2..=n {
        let factors = break_down_into_prime_factors(i as u64, &primes);

        if factors.len() == num_factors {
            previous_factors.push_back(factors);

            // It's quite possible that we already have the answer, as the consecutive numbers might have co-prime
            // factors by construction. While this is the case for ``num_factors`` <= 4, I'm not sure if it's true
            // for larger values of ``num_factors``, so we do test that they are all unique.

            if previous_factors.len() == num_factors {
                // Test that the previous factors are all different
                let unique_factors = previous_factors.iter().fold(
                    HashSet::with_capacity(num_factors.pow(2)), |acc: HashSet<u64>, a| {
                        acc.union(a).copied().collect::<HashSet<_>>()
                    }
                ).len();

                if unique_factors == num_factors*num_factors {
                    return Some(i-num_factors+1);
                }
            }
        } else {
            previous_factors.clear();
        }
    }

    None
}

fn get_primes_up_to(n: usize) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n + 1];

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i as u64);

            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    primes
}

/// Breaks down a number into its prime factors.
///
/// We return the prime numbers multiplied by their powers, e.g. 644 = 2^2 * 7 * 23 = {4, 7, 23} to match their
/// use in the problem.
fn break_down_into_prime_factors(n: u64, primes: &[u64]) -> HashSet<u64> {
    let mut factors = HashSet::new();
    let mut n = n;

    let limit = (n as f64).sqrt() as u64;

    for prime in primes {
        if *prime > limit {
            break;
        }
        let mut cumulative = 1;

        while n % prime == 0 {
            // Insert the prime factor to its power
            n /= prime;
            cumulative *= prime;

            if n == 1 {
                // If we have reached 1, then we have found all the prime factors
                factors.insert(cumulative);
                return factors;
            }
        }

        if cumulative > 1 {
            factors.insert(cumulative);
        }
    }

    // If we get here, then we have a prime factor larger than the square root of n.
    factors.insert(n);
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(14, &[2, 7])]
    #[test_case(644, &[4, 7, 23])]
    #[test_case(646, &[2, 17, 19])]
    fn test_break_down_into_prime_factors(n: u64, factors: &[u64]) {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23];
        let actual = break_down_into_prime_factors(n, &primes);

        let expected = factors.iter().copied().collect::<HashSet<_>>();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_prime_generation() {
        let actual = get_primes_up_to(23);
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23];

        assert_eq!(actual, primes)
    }

    #[test_case(20, 14, 2)]
    #[test_case(700, 644, 3)]
    fn test_get_consecutive_prime_factors(max_n: usize, expected: usize, num_factors: usize) {
        let actual = get_consecutive_prime_factors(max_n, num_factors);

        assert_eq!(actual, Some(expected))
    }
}