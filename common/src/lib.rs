use num::traits::{PrimInt, Unsigned};
pub fn get_primes_up_to(n: usize) -> Vec<u64> {
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

pub fn get_digits_sorted<T>(num: T) -> Vec<T>
    where T: std::ops::DivAssign + PrimInt + Copy + From<u8>
{
    let mut digits = Vec::new();
    let mut num = num;
    let ten = 10.into();

    while num > T::zero() {
        digits.push(num % ten);
        num /= ten;
    }
    digits.sort_unstable();
    digits
}

pub fn get_digits(num: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut num = num;

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits.reverse();
    digits
}

pub fn get_digits_gen<T>(num: T) -> Vec<T> where
    T: std::ops::DivAssign + PrimInt + Copy + From<u8>
{
    let mut digits = Vec::new();
    let mut num = num;
    let ten = 10.into();

    while num > T::zero() {
        digits.push(num % ten);
        num /= ten;
    }

    digits.reverse();
    digits
}
