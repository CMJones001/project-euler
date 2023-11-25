use problem_23::get_abundant_numbers::brute_force::collect_abundant_numbers_parallel;

fn main() {
    let num_max = 20161;

    let non_abundant_sum = get_non_abundant_sums(num_max);
    println!("{:?}", non_abundant_sum)
}

fn get_non_abundant_sums(num_max: u64) -> u64 {
    let abundant_numbers = collect_abundant_numbers_parallel(num_max);
    let mut numbers: Vec<_> = (1..=num_max).collect();

    for a in abundant_numbers.iter() {
        for b in abundant_numbers.iter() {
            if b > a || a + b > num_max {
                break;
            }
            numbers[(a + b - 1) as usize] = 0;
        }
    }

    numbers.iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_abundant_numbers() {
        let num_max = 20161;

        let non_abundant_sum = get_non_abundant_sums(num_max);
        assert_eq!(non_abundant_sum, 4179871)
    }
}
