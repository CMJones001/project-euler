use std::collections::HashSet;

/// Problem 44
///
/// Find the pair of pentagonal numbers, P_j and P_k, for which their sum and difference are
/// pentagonal and D = |P_k - P_j| is minimised; what is the value of D?
///
/// At least for the first 100_000 pentagonal numbers there is only one pair that satisfies this
/// condition, namely (7042750, 1560090) with D = 5482660

fn main() {
    let max_index: usize = 100000;
    let pentagonals: Vec<u64> = (1..=max_index).map(|v| get_pentagonal(v) as u64).collect();
    let mut pentagonal_set: HashSet<_> = pentagonals.clone().into_iter().collect();

    let mut min_diff = u64::MAX;
    let mut delta_steps: Option<usize> = None;

    for (i, pent_i) in pentagonals.iter().enumerate() {

        // We can start at i+1 because we know that the difference will be positive
        // We can also stop once we have taken more steps than the current minimum difference
        // As this will only increase the difference

        let range = if let Some(steps) = delta_steps {
            let start = i+1;
            let end = std::cmp::min(start + steps, pentagonals.len());
            start..end
        } else {
            i+1..pentagonals.len()
        };

        // Could do this with a ``find`` but this is more readable
        for (j, pent_j) in pentagonals[range].iter().enumerate() {
            let sum = pent_i + pent_j;
            let diff = pent_j - pent_i;

            if pentagonal_set.contains(&sum) && pentagonal_set.contains(&diff) {
                if diff < min_diff {
                    min_diff = diff;
                }
                delta_steps = Some(j);
                // Could also break here as there's only one pair that satisfies this condition
                continue; // Once we find a diff, we don't need to continue for this ``j``
            }
        }
    }

    println!("min_diff: {}", min_diff);
}

fn get_pentagonal(n: usize) -> usize {
    n * (3 * n - 1) / 2
}