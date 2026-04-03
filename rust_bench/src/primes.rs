/// Sieve of Eratosthenes: sum all primes up to `limit`.
/// O(N log log N) time, O(N) space.
pub fn sum_primes_sieve(limit: usize) -> u64 {
    if limit < 2 {
        return 0;
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2usize;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(i, _)| i as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_up_to_10() {
        // 2+3+5+7 = 17
        assert_eq!(sum_primes_sieve(10), 17);
    }

    #[test]
    fn primes_up_to_2() {
        assert_eq!(sum_primes_sieve(2), 2);
    }
}
