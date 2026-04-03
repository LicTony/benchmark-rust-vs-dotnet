/// LCG-based reproducible random array generator (seed 42).
/// Uses the same constants as Java's `java.util.Random` for portability.
pub fn generate_array(seed: u64, size: usize) -> Vec<u32> {
    let mut state = seed;
    (0..size)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 33) as u32
        })
        .collect()
}

/// Sort a `u32` array in-place using Rust's `sort_unstable` (pdqsort).
pub fn sort_array(mut arr: Vec<u32>) -> Vec<u32> {
    arr.sort_unstable();
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_is_ordered() {
        let arr = generate_array(42, 1000);
        let sorted = sort_array(arr.clone());
        assert!(sorted.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(sorted.len(), arr.len());
    }
}
