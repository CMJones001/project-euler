fn main() {
    let power = 5;
    let max_val = 400000;

    let power_values: u32 = (2..=max_val).filter(|&x|
        x == digit_power_sum(x, power)
    ).sum();

    println!("Power {} values: {}", power, power_values);

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
