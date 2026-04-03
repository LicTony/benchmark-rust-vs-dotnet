/// Generate an N×N matrix of f64 values using an LCG (seed-based, reproducible).
/// Values are in [0.0, 1.0).
pub fn generate_matrix(seed: u64, n: usize) -> Vec<Vec<f64>> {
    let mut state = seed;
    (0..n)
        .map(|_| {
            (0..n)
                .map(|_| {
                    state = state
                        .wrapping_mul(6_364_136_223_846_793_005)
                        .wrapping_add(1_442_695_040_888_963_407);
                    (state >> 33) as f64 / u32::MAX as f64
                })
                .collect()
        })
        .collect()
}

/// Naïve matrix multiplication — O(N³), cache-friendly ikj loop order.
/// Equivalent Big-O to the C# implementation.
pub fn multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut c = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for k in 0..n {
            let a_ik = a[i][k];
            for j in 0..n {
                c[i][j] += a_ik * b[k][j];
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_2x2() {
        // identity * identity = identity
        let id = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let result = multiply(&id, &id);
        assert!((result[0][0] - 1.0).abs() < 1e-10);
        assert!((result[1][1] - 1.0).abs() < 1e-10);
        assert!((result[0][1]).abs() < 1e-10);
    }
}
