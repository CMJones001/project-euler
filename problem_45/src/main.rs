use std::time::Instant;
// Problem 45: Triangular, pentagonal, and hexagonal

// We note that T(2n-1) = H(n), so all hexagonal numbers are triangular, so we
// only need to check pentagonal and hexagonal numbers.

// We are provided that H(143) = 40755 is the first hexagonal number that is
// also pentagonal. We can use this to start our search and for testing.

fn hexagonal(n: u64) -> u64 {
    n * (2 * n - 1)
}

fn main() {
    // If test_run is true we should find H(143) = 40755 as the test case
    let test_run = false;
    let starting_value = if test_run { 2 } else { 144 };

    let start = Instant::now();
    let result = (starting_value..).map(hexagonal).find(|&n| is_pentagonal(n)).unwrap();
    let elapsed = start.elapsed();

    println!("{}", result);
    eprintln!("Elapsed: {:2} μs", elapsed.as_micros());

    if test_run && result != 40755{
        eprintln!("The test has failed, expected 40755, got {}", result);
    }
}

fn is_pentagonal(n: u64) -> bool {
    // From Wikipedia, if x is an integer, then n is pentagonal
    let x = (1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0;
    x == x.floor()
}