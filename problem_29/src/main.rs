use std::collections::HashSet;
use std::ops::Rem;

const PRIMES: [u16; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

fn main() {
    let max_value = 100;
    let max_power = 100;

    let n_unique = get_unique_values(max_value, max_power);
    println!("There are {n_unique} entries.");
}

/// Return the number of unique values of a^b for a = [2, max_value] and b = [2, max_power]
///
/// We make use of the fact that raising a number to a power is a multiplication of the powers in
/// the prime factorisation of the number. We make use of this to break all the numbers in
/// [2, max_value] into vectors of the powers of the prime factors and store these vectors
/// and their multiples over [2, max_power] in a set.
fn get_unique_values(max_value: u16, max_power: u16) -> usize {
    let mut set = HashSet::new();
    let starting_values = 2..=max_value;

    for value in starting_values {
        let prime_factors = break_down_into_prime_factors(value);

        // Raising a number to a power ``n`` simply multiplies the powers of prime factors by ``n``.
        for power in 2..=max_power {
            let power_multiple: Vec<u16> = prime_factors.iter().map(|&v| v * power).collect();
            set.insert(power_multiple);
        }
    }

    set.len()
}

/// Break a number down into its prime factors
///
/// Returns an array with the ``i``th entry the power of the given prime (given in ``PRIMES``).
/// As we've hardcoded the primes, this will only work with values up to 100.
fn break_down_into_prime_factors(value: u16) -> [u16; 25] {
    if value > 100 {
        panic!("Number must be smaller than 100!");
    }

    let mut value = value;
    let mut prime_factors = [0; 25];

    for (power, prime) in prime_factors.iter_mut().zip(PRIMES) {
        loop {
            if value.rem(prime) != 0 {
                break
            }
            value /= prime;
            *power += 1;
        }
    }

    prime_factors
}

#[cfg(test)]
mod test {
    use super::*;

    /// Debugging function for displaying the prime decomposition vector
    fn pretty_format_factors(factor_array: &[u16], starting_val: u16) -> String {
        let factor_string = factor_array
            .iter()
            .enumerate()
            .filter(|(_, &val)| val != 0)
            .map(|(index_, &val)| {
                format!("{}^{val}", PRIMES[index_])
            })
            .reduce(|a, b| format!("{a} × {b}"))
            .unwrap();

        format!("{starting_val} = {factor_string}")
    }

    #[test]
    fn test_print_primes() {
        let value = 90;
        let prime_factors = break_down_into_prime_factors(value);
        let pretty_fmt = pretty_format_factors(&prime_factors, value);

        let pretty_fmt_expected = "90 = 2^1 × 3^2 × 5^1".to_string();
        assert_eq!(pretty_fmt, pretty_fmt_expected)
    }

    #[test]
    fn power_breakdown_4() {
        let prime_factors = break_down_into_prime_factors(4);

        let mut prime_factors_expected = [0; 25];
        prime_factors_expected[0] = 2;

        assert_eq!(prime_factors, prime_factors_expected);
    }

    #[test]
    fn power_breakdown_12() {
        let prime_factors = break_down_into_prime_factors(12);

        let mut prime_factors_expected = [0; 25];
        prime_factors_expected[0] = 2;
        prime_factors_expected[1] = 1;

        assert_eq!(prime_factors, prime_factors_expected);

        let total: u16 = prime_factors
            .into_iter()
            .enumerate()
            .map(|(index_, value)| PRIMES[index_].pow(value as u32))
            .product();
        assert_eq!(total, 12);
    }

    #[test]
    fn example_solution() {
        let num_entries = get_unique_values(5, 5);
        let num_entries_expected = 15;

        assert_eq!(num_entries, num_entries_expected);
    }

    #[test]
    fn full_solution() {
        let num_entries = get_unique_values(100, 100);
        let num_entries_expected = 9183;

        assert_eq!(num_entries, num_entries_expected);
    }
}
