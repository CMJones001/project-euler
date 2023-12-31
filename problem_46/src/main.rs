use std::time::Instant;
// Problem 46: Goldbach's other conjecture

// Question:
// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and
// twice a square.

// It turns out that the conjecture was false.
// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?


fn main() {
    let now = Instant::now();
    let result = (3..).step_by(2)
        .find(|&n| !test_goldbach(n)).unwrap();
    let elapsed = now.elapsed();

    println!("The smallest odd composite that cannot be written as the sum of a prime and twice a square is {}", result);
    println!("Elapsed: {:.2} μs", elapsed.as_micros());
}

/// Return if the number can be written as the sum of a prime and twice a square
fn test_goldbach(n: u32) -> bool {
    if is_prime(n) {
        // If n is prime then we say it is trivially true
        return true;
    }

    // We only need to check up to the square root of n/2
    // We might have chosen to check if N - P for some prime P is a square, but that would involve far more checks.
    let limit = ((n as f64)/2.0).sqrt() as u32;
    (1..=limit).any(|i| is_prime(n - 2 * i * i))
}


fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32;
    !(5..=limit)
        .step_by(6)
        .any(|i| n % i == 0 || n % (i + 2) == 0)
}

