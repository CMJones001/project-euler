use std::collections::VecDeque;

fn main() {
    let digits = generate_candidates();
    let res: Vec<_> = digits.iter().map(|v| sum_digits(v)).collect();

    let n_digits = res.iter().count();

    println!("Number of truncatable primes: {}", n_digits);
    println!("{res:?}")
}

#[derive(Debug, Clone)]
struct TruncateCandidate {
    digits: VecDeque<u64>,
}

impl TruncateCandidate {
    fn from_slice(slice: &[u64]) -> Self {
        let digits = VecDeque::from(slice.to_owned());
        TruncateCandidate { digits }
    }

    fn to_int(self) -> u64 {
        self.digits
            .iter()
            .enumerate()
            .map(|(num, val)| 10_u64.pow(num as u32) * val)
            .sum()
    }
}

fn get_primes_and_candidates(vecs: Vec<Vec<u64>>) -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
    let primes = vecs.clone().into_iter().map(add_valid_digit_end).flatten().filter(|v| is_left_prime(v));
    let candidates = vecs.into_iter().map(add_valid_digit_mid).flatten();

    (primes.collect(), candidates.collect())
}

fn generate_candidates() -> Vec<Vec<u64>> {
    let first_digits = vec![Vec::from([3]), Vec::from([5]), Vec::from([7])];

    let two_digit_primes = first_digits
        .clone()
        .into_iter()
        .map(add_valid_digit_end)
        .flatten()
        .filter(|v| is_left_prime(v));

    let two_digit_primes_candidates = first_digits
        .clone()
        .into_iter()
        .map(add_valid_digit_mid)
        .flatten();

    let three_digit_primes = two_digit_primes_candidates
        .clone()
        .into_iter()
        .map(add_valid_digit_end)
        .flatten()
        .filter(|v| is_left_prime(v));

    let three_digit_candidates = two_digit_primes_candidates
        .into_iter()
        .map(add_valid_digit_mid)
        .flatten();

    let four_digit_primes = three_digit_candidates
        .clone()
        .into_iter()
        .map(add_valid_digit_end)
        .flatten()
        .filter(|v| is_left_prime(v));

    let four_digit_candidates = three_digit_candidates
        .into_iter()
        .map(add_valid_digit_mid)
        .flatten();

    let five_digit_primes = four_digit_candidates
        .clone()
        .into_iter()
        .map(add_valid_digit_end)
        .flatten()
        .filter(|v| is_left_prime(v));

    let five_digit_candidates = four_digit_candidates
        .into_iter()
        .map(add_valid_digit_mid)
        .flatten();

    let six_digit_primes = five_digit_candidates
        .clone()
        .into_iter()
        .map(add_valid_digit_end)
        .flatten()
        .filter(|v| is_left_prime(v));

    two_digit_primes
        .chain(three_digit_primes)
        .chain(four_digit_primes)
        .chain(five_digit_primes)
        .chain(six_digit_primes)
        .collect()
}

/// Test for primalness of all the left-truncated numbers
///
/// Removes the digits from the left and checks if the remaining number is prime
fn is_left_prime(v: &[u64]) -> bool {
    let n_digits = v.len();

    (1..n_digits).all(|i| is_prime(sum_digits(&v[i..])))
}

fn are_digits_prime(v: &[u64]) -> bool {
    is_prime(sum_digits(v))
}

fn sum_digits(v: &[u64]) -> u64 {
    v.iter()
        .enumerate()
        .map(|(index, val)| 10_u64.pow(index as u32) * val)
        .sum()
}

fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    !(5..=limit)
        .step_by(6)
        .any(|i| n % i == 0 || n % (i + 2) == 0)
}

fn add_valid_digit_end(value: Vec<u64>) -> Vec<Vec<u64>> {
    let current_digits = value.len();

    let mut candidates = Vec::new();
    let mut candidate = value.clone();
    candidate.push(0);

    // This can include the digits 2 an 5 as these are only valid in the left-most digit position
    for i in [2, 3, 5, 7] {
        candidate[current_digits] = i;

        if is_valid(&candidate) && are_digits_prime(&candidate) {
            candidates.push(candidate.clone());
        }
    }

    candidates
}

fn add_valid_digit_mid(value: Vec<u64>) -> Vec<Vec<u64>> {
    let current_digits = value.len();

    let mut candidates = Vec::new();
    let mut candidate = value.clone();
    candidate.push(0);

    for i in [1, 3, 7, 9] {
        candidate[current_digits] = i;

        if is_valid(&candidate) && are_digits_prime(&candidate) {
            candidates.push(candidate.clone());
        }
    }

    candidates
}

/// A quick test to remove values that can't be prime
///
/// For now we check if the number is divisible by three, but this can be made more complex
fn is_valid(candidate: &Vec<u64>) -> bool {
    let digit_sum: u64 = candidate.iter().sum();
    if digit_sum % 3 == 0 {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    fn create_deques(vecs: Vec<&[u64]>) -> Vec<Vec<u64>> {
        let mut main_vec = Vec::new();

        for v in vecs.into_iter() {
            main_vec.push(Vec::from(v.to_owned()));
        }

        main_vec
    }

    #[test]
    fn add_valid_digits_1() {
        let digits = Vec::from([1]);

        let valid_candidates = add_valid_digit_mid(digits);

        let expected_digits = create_deques(vec![&[1, 1], &[1, 3], &[1, 7]]);

        assert_eq!(valid_candidates, expected_digits)
    }

    #[test]
    fn add_valid_digits_3() {
        let digits = Vec::from([3]);

        let valid_candidates = add_valid_digit_mid(digits);

        let expected_digits = create_deques(vec![&[3, 1], &[3, 7]]);

        assert_eq!(valid_candidates, expected_digits)
    }

    #[test]
    fn add_valid_digits_7() {
        let digits = Vec::from([7]);

        let valid_candidates = add_valid_digit_mid(digits);

        let expected_digits = create_deques(vec![&[7, 1], &[7, 3], &[7, 9]]);

        assert_eq!(valid_candidates, expected_digits)
    }

    #[test]
    fn add_valid_digits_9() {
        let digits = Vec::from([9]);

        let valid_candidates = add_valid_digit_mid(digits);

        let expected_digits = create_deques(vec![&[9, 1], &[9, 7]]);

        assert_eq!(valid_candidates, expected_digits)
    }


    #[test]
    fn add_final_digits() {
        let digits = Vec::from([7, 9, 7]);
        let valid_candidates = add_valid_digit_end(digits);

        // 5797 is not prime
        let expected_digits = create_deques(vec![&[7, 9, 7, 2], &[7, 9, 7, 3]]);

        assert_eq!(valid_candidates, expected_digits)
    }

    #[test]
    fn test_adding_digits() {
        let starting_digit = Vec::from([7]);
        let valid_candidates = add_valid_digit_mid(starting_digit);

        let expected_candidate = vec![7, 9];
        assert!(valid_candidates.contains(&expected_candidate));

        let starting_digit = expected_candidate;
        let valid_candidates = add_valid_digit_end(starting_digit);

        let expected_candidate = vec![7, 9, 7];
        assert!(valid_candidates.contains(&expected_candidate));

        let starting_digit = expected_candidate;
        let valid_candidates = add_valid_digit_end(starting_digit);

        let expected_candidate = vec![7, 9, 7, 3];
        assert!(valid_candidates.contains(&expected_candidate));
    }

    #[test]
    fn test_primes() {
        let primes = vec![
            373, 563, 593, 607, 653, 733, 947, 977, 1103, 1123, 1187, 1223, 1367, 1511, 1747, 1753,
            1907, 2287, 2417, 2677, 2903, 2963, 3307, 3313,
        ];
        assert!(primes.into_iter().all(is_prime))
    }

    #[test]
    fn is_composite() {
        let composite = vec![
            4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32, 33, 34, 35,
            36, 38, 39, 40, 42, 44, 45, 46, 48, 49, 50, 51, 52, 54, 55, 56, 57, 58, 60, 62, 63, 64,
            65, 66, 68, 69, 70, 72, 74, 75, 76, 77, 78, 80, 81, 82, 84, 85, 86, 87, 88, 90, 91, 92,
            93, 94, 95, 96, 98, 99, 100, 102, 104, 105, 106, 108, 110, 111,
        ];

        assert!(!composite.into_iter().any(is_prime));
    }

    #[test]
    fn test_is_left_prime() {
        let digits = [7, 9, 7, 3];

        assert!(are_digits_prime(&digits));
        assert!(is_left_prime(&digits));
    }


    #[test]
    fn test_sum_digits() {
        let digits = vec![vec![1, 9, 5, 6], vec![7, 9, 8, 6]];
        let expected_sums = vec![6591, 6897];

        for (digits, expected) in digits.iter().zip(expected_sums) {
            let total = sum_digits(digits);
            assert_eq!(total, expected);
        }
    }
}
