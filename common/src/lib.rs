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

