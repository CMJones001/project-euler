/*
Problem 49 - Prime permutations

The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is
unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers
are permutations of one another.
There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
property, but there is one other 4-digit increasing sequence.

Notes:

It turns out the other sequence also increases by 3330, this would allow for a faster solution,
but I'm not sure why this would be the case, so I'm going to leave it as is for now, as this would seem like
cheating.
This would seem to "rotate" the first three digits, leaving the last digit in place. However, I'm still unsure
why this would need to be the case.
*/


use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let map: HashMap<_, _> = split_digits()
        .into_iter()
        .filter(|(_, v)| v.len() >= 3)
        .collect();

    let results = remove_duplicates(map);
    let time = now.elapsed();

    for res in results {
        println!("{} {} {}", res.0, res.1, res.2);
    }

    println!("Time: {} μs", time.as_micros());

}

fn remove_duplicates(digit_map: HashMap<Digits, Vec<u32>>) -> Vec<(u32, u32, u32)>{
    let mut results = Vec::new();

    for (_digits, primes) in digit_map {

        // Get a hashmap of the distance between each pair of primes, grouped by distance
        let distance_map: HashMap<_, _> = primes
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| {
                // Skip the values we have already seen
                // While ``combination`` is easier to read, it is slower than ``skip``
                primes.iter().skip(i + 1).map(move |p2| {
                    let prime = PrimePair {
                        p1: *p1,
                        p2: *p2,
                        distance: *p2 - *p1,
                    };
                    (prime.distance, prime)
                })
            })
            .into_group_map();

        // Find the pairs that overlap
        //
        // Sometimes we have two pairs that have the same distances but are not connected, so we need
        // to filter those out
        for (_distance, pairs) in distance_map {
            pairs.iter().enumerate().for_each(|(i, pair)| {

                let pairs = pairs.iter().find(|p_lower_| p_lower_.is_upper_limit(&pair));
                if let Some(p_lower) = pairs {
                    results.push((p_lower.p1, p_lower.p2, pair.p2));
                }
            });
        }
    }

    results
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct PrimePair {
    p1: u32,
    p2: u32,
    distance: u32,
}

impl PrimePair {
    fn is_upper_limit(&self, other: &Self) -> bool {
        self.p2 == other.p1
    }
}

impl Display for PrimePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.p1, self.p2, self.distance)
    }
}

/// Create a hashmap of digits to primes
///
/// The key is the (sorted) digits of the prime, and the value is a vector of all the primes that
/// have those digits, sorted by value
fn split_digits() -> HashMap<Digits, Vec<u32>> {
    let primes = get_primes_up_to(10000)
        .into_iter()
        .skip_while(|n| *n < 1000);

    primes.map(|n| {
        let digits = Digits::from_number(n as u32);
        (digits, n as u32)
    }).into_group_map()
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

/// Sorted storage of digits from four digit number
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Digits {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Digits {
    fn from_number(number: u32) -> Digits {
        let a = number / 1000;
        let b = (number - a * 1000) / 100;
        let c = (number - a * 1000 - b * 100) / 10;
        let d = number - a * 1000 - b * 100 - c * 10;

        let mut digits = [a, b, c, d];
        digits.sort();

        Digits {
            a: digits[0],
            b: digits[1],
            c: digits[2],
            d: digits[3],
        }
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.a, self.b, self.c, self.d)
    }
}
