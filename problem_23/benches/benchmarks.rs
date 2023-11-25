use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use problem_23::proper_divisors::{
    calculate_proper_divisors, calculate_proper_divisors_doubled, calculate_proper_divisors_sqrt,
};

use problem_23::get_abundant_numbers::brute_force::collect_abundant_numbers_parallel;
use problem_23::get_abundant_numbers::multiplicative_approach::collect_abundant_numbers;

fn full_proper_divisors(c: &mut Criterion) {
    let value = 10236;

    c.bench_function("full divisors", |b| {
        b.iter(|| calculate_proper_divisors(black_box(value)).collect::<Vec<_>>())
    });

    c.bench_function("sqrt divisors", |b| {
        b.iter(|| calculate_proper_divisors_sqrt(black_box(value)))
    });

    c.bench_function("sqrt divisors alt", |b| {
        b.iter(|| calculate_proper_divisors_doubled(black_box(value)))
    });
}

fn get_abundant_numbers_bench(c: &mut Criterion) {
    let value = 20161;

    c.bench_function("Brute Force", |b| {
        b.iter(|| collect_abundant_numbers_parallel(value))
    });

    c.bench_function("Multiplicative", |b| {
        b.iter(|| collect_abundant_numbers(value))
    });
}

fn remove_loop(abundant_numbers: Vec<u64>) -> Vec<u64> {
    let mut numbers: Vec<_> = (1..=20161).collect();

    for a in abundant_numbers.iter() {
        for b in abundant_numbers.iter() {
            if b > a || a + b > 20161 {
                break;
            }
            numbers[(a + b - 1) as usize] = 0;
        }
    }

    numbers
}

fn remove_combinations(abundant_numbers: Vec<u64>) -> Vec<u64> {
    let mut numbers: Vec<_> = (1..=20161).collect();

    abundant_numbers
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a + b)
        .chain(abundant_numbers.iter().map(|x| 2 * x))
        .filter(|x| *x <= 20161)
        .for_each(|x| numbers[(x - 1) as usize] = 0);

    numbers
}

fn remove_non_overlapping(c: &mut Criterion) {
    let max_val = 20161;
    let abundant_numbers = collect_abundant_numbers_parallel(max_val);

    c.bench_function("remove_loop", |b| {
        b.iter(|| remove_loop(black_box(abundant_numbers.clone())))
    });

    c.bench_function("remove_combinations", |b| {
        b.iter(|| remove_combinations(black_box(abundant_numbers.clone())))
    });
}

criterion_group!(
    collections,
    remove_non_overlapping,
    full_proper_divisors,
    get_abundant_numbers_bench
);
criterion_main!(collections);
