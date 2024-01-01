use std::time::Instant;
// Problem 48: Self powers

// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.

// This will obviously overflow a 64-bit integer, so we'll make use of the
// modulo exp property: (a * b) % c = ((a % c) * (b % c)) % c

fn main() {
    let max = 1000;

    let start = Instant::now();
    let sum = truncated_power_series_sum(max);
    let duration = start.elapsed();
    let sum = sum % 10_000_000_000;

    println!("{}", sum);
    println!("Time taken is: {:?}", duration);
}

fn truncated_power_series_sum(max: u64) -> u64 {
    let sum = (1..=max)
        .map(|x| memory_efficient_mod_exp(x, x, 10_000_000_000))
        .reduce(|acc, b| (acc + b) % 100_000_000_000).unwrap();

    sum
}

// This is a memory efficient version of the mod_exp function, but it's
// slower than the right-to-left version. Doesn't overflow for large values.
fn memory_efficient_mod_exp(base: u64, exp: u64, modulo: u64) -> u64 {
    if modulo == 1 {
        return 0;
    }

    (0..exp).fold(1, |acc, a| (acc * base) % modulo)
}

// This is a more efficient version of the above function, but it overflows
// for large values.
fn right_to_left_mod_exp(base: u64, exp: u64, modulo: u64) -> u64 {
    if modulo == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = base % modulo;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }

        exp >>= 1;
        base = (base * base) % modulo;
    }

    result
}
