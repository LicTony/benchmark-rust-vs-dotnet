namespace CSharpBench.Core;

/// <summary>
/// Naïve matrix multiplication: O(N³), ikj loop order for cache efficiency.
/// Uses double[,] (contiguous 2-D array) — equivalent to Rust's Vec&lt;Vec&lt;f64&gt;&gt;.
/// </summary>
public static class MatrixEngine
{
    public static double[,] GenerateMatrix(int seed, int n)
    {
        var rng = new Random(seed);
        var m = new double[n, n];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                m[i, j] = rng.NextDouble();
        return m;
    }

    public static double[,] Multiply(double[,] a, double[,] b, int n)
    {
        var c = new double[n, n];
        for (int i = 0; i < n; i++)
        {
            for (int k = 0; k < n; k++)
            {
                double a_ik = a[i, k];
                for (int j = 0; j < n; j++)
                    c[i, j] += a_ik * b[k, j];
            }
        }
        return c;
    }
}
