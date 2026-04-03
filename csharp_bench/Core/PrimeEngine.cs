namespace CSharpBench.Core;

/// <summary>
/// Sieve of Eratosthenes — same algorithm as the Rust implementation.
/// O(N log log N) time, O(N) space.
/// </summary>
public static class PrimeEngine
{
    public static long SumPrimesSieve(int limit)
    {
        if (limit < 2) return 0;

        var isComposite = new bool[limit + 1];
        long sum = 0;

        for (int i = 2; i <= limit; i++)
        {
            if (!isComposite[i])
            {
                sum += i;
                // Mark multiples starting from i*i
                for (long j = (long)i * i; j <= limit; j += i)
                    isComposite[j] = true;
            }
        }

        return sum;
    }
}
