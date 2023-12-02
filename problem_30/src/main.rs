use itertools::{repeat_n, Itertools};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let power_val = power_approach();
    println!("Time taken: {}ms", now.elapsed().as_millis());
    println!("Power approach: {}", power_val);

    // Brute force approach
    let now = Instant::now();
    let bf_val = brute_force_approach(5);
    println!("Time taken: {}ms", now.elapsed().as_millis());
    println!("Brute force approach: {}", bf_val);

    let now = Instant::now();
    let explicit_val = explicit_powers();
    println!("Time taken: {}ms", now.elapsed().as_millis());
    println!("Explicit approach: {}", explicit_val);
}

fn power_approach() -> u32 {
    let digit_powers = (0..10).map(|x: u32| (x, x.pow(5))).collect::<Vec<_>>();

    let powers = (0..=6).map(|x| 10_u32.pow(x)).collect::<Vec<_>>();

    let mut power_approach: u32 = repeat_n(digit_powers, 6)
        .multi_cartesian_product()
        .filter_map(|val| {
            let sum = val.iter().map(|(_, power)| power).sum::<u32>();
            if sum > 999999 {
                return None;
            }

            let number = val
                .iter()
                .zip(powers.iter())
                .map(|((digit, _), power)| power * digit)
                .sum::<u32>();

            if number == sum {
                Some(number)
            } else {
                None
            }
        })
        .sum();

    power_approach -= 1; // 1 is not a sum
    power_approach
}

fn explicit_powers() -> u32 {
    // Manually unrolled loops
    //
    // Far faster than the other approaches, but I still hate it
    let digit_powers = (0..10).map(|x: u32| (x, x.pow(5))).collect::<Vec<_>>();
    let mut matches = 0;
    let max_val = 400000;

    for (num_a, pow_a) in digit_powers.iter() {
        for (num_b, pow_b) in digit_powers.iter() {
            for (num_c, pow_c) in digit_powers.iter() {
                for (num_d, pow_d) in digit_powers.iter() {
                    let abcd = pow_a + pow_b + pow_c + pow_d;
                    if abcd > max_val {
                        continue;
                    }
                    for (num_e, pow_e) in digit_powers.iter() {
                        let abcde = abcd + pow_e;
                        if abcde > max_val {
                            continue;
                        }
                        for (num_f, pow_f) in digit_powers.iter() {
                            let abcdef = abcde + pow_f;
                            if abcdef > max_val {
                                continue;
                            }

                            let number = num_a * 100000
                                + num_b * 10000
                                + num_c * 1000
                                + num_d * 100
                                + num_e * 10
                                + num_f;
                            if number == abcdef {
                                matches += number;
                            }
                        }
                    }
                }
            }
        }
    }

    matches - 1 // 1 is not a sum
}

fn brute_force_approach(power: u32) -> u32 {
    let max_val = 400000;

    let power_values: u32 = (2..=max_val)
        .filter(|&x| x == digit_power_sum(x, power))
        .sum();

    power_values
}

fn digit_power_sum(num: u32, power: u32) -> u32 {
    let digits = to_digits_div(num);
    digits.iter().map(|x| x.pow(power)).sum()
}

fn to_digits(num: u32) -> Vec<u32> {
    // Break the number into digits
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

fn to_digits_div(num: u32) -> Vec<u32> {
    let n_digits = (num as f32).log10() as u32 + 1;
    let mut digits = Vec::with_capacity(n_digits as usize);
    let mut n = num;

    for _ in 0..n_digits {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}
