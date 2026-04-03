namespace CSharpBench.Core;

/// <summary>
/// Sorting engine using the same LCG seed as the Rust implementation (seed 42).
/// Uses Array.Sort which is an introspective sort (similar to pdqsort in Rust).
/// </summary>
public static class SortEngine
{
    /// <summary>Generate a reproducible array of <paramref name="size"/> random integers.</summary>
    public static int[] GenerateArray(int seed, int size)
    {
        var rng = new Random(seed);
        var arr = new int[size];
        for (int i = 0; i < size; i++)
            arr[i] = rng.Next();
        return arr;
    }

    /// <summary>Sort a copy of the input array and return it.</summary>
    public static int[] Sort(int[] input)
    {
        var copy = (int[])input.Clone();
        Array.Sort(copy);
        return copy;
    }
}
