use std::time::Instant;

fn main() {
    let now = Instant::now();
    let explicit_val = explicit_factorials();
    println!("Time taken: {} µs", now.elapsed().as_micros());
    println!("Explicit approach: {}", explicit_val);
}

fn factorial(x: u32) -> u32 {
    (1..=x).product()
}

/// Returns a+b if enable is true, else returns a
fn conditional_add(a: u32, b: u32, enable: bool) -> u32 {
    if enable {
        a + b
    } else {
        a
    }
}

fn explicit_factorials() -> u32 {
    // Manually unrolled loops
    //
    // Far faster than the other approaches I tried, but I still hate it
    //
    // As 0! is one, we need to "disable" the higher digits if they are leading zeros
    let digit_powers = (0..10).map(|x: u32| (x, factorial(x))).collect::<Vec<_>>();
    let mut matches = 0;

    let mut enabled_a = false;
    let mut enabled_b = false;
    let mut enabled_c = false;
    let mut enabled_d = false;
    let mut enabled_e = false;

    for (num_a, pow_a) in digit_powers.iter() {
        let digit_a = num_a * 100_000;

        if *num_a > 0 {
            enabled_a = true;
        }
        let power_a = conditional_add(0, *pow_a, enabled_a);

        for (num_b, pow_b) in digit_powers.iter() {
            let digit_b = digit_a + num_b * 10_000;

            if *num_b > 0 {
                enabled_b = true;
            }
            let power_b = conditional_add(power_a, *pow_b, enabled_b);

            for (num_c, pow_c) in digit_powers.iter() {
                let digit_c = digit_b + num_c * 1_000;

                if *num_c > 0 {
                    enabled_c = true;
                }
                let power_c = conditional_add(power_b, *pow_c, enabled_c);

                for (num_d, pow_d) in digit_powers.iter() {
                    let digit_d = digit_c + num_d * 100;

                    if *num_d > 0 {
                        enabled_d = true;
                    }
                    let power_d = conditional_add(power_c, *pow_d, enabled_d);

                    for (num_e, pow_e) in digit_powers.iter() {
                        let digit_e= digit_d + num_e * 10;

                        if *num_e > 0 {
                            enabled_e = true;
                        }
                        let power_e = conditional_add(power_d, *pow_e, enabled_e);

                        for (num_f, pow_f) in digit_powers.iter() {
                            let power = power_e + pow_f;
                            let digit = digit_e + num_f * 1;

                            if digit == power {
                                matches += digit;
                            }
                        }
                    }
                }
            }
        }
    }

    matches - 3 // 1! and 2! are not a sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0), "0! = 1");
        assert_eq!(1, factorial(1), "1! = 1");
        assert_eq!(24, factorial(4), "1! = 1");
        assert_eq!(120, factorial(5), "5! = 120");
        assert_eq!(362_880, factorial(9), "5! = 120");
    }
}