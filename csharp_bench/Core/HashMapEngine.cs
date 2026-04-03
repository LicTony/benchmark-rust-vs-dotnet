namespace CSharpBench.Core;

/// <summary>
/// HashMap operations using Dictionary&lt;long, long&gt; — equivalent to Rust's std::collections::HashMap.
/// </summary>
public static class HashMapEngine
{
    /// <summary>Insert <paramref name="count"/> entries: key = i, value = i * i.</summary>
    public static Dictionary<long, long> InsertEntries(int count)
    {
        var dict = new Dictionary<long, long>(count);
        for (long i = 0; i < count; i++)
            dict[i] = i * i;
        return dict;
    }

    /// <summary>Look up all <paramref name="count"/> keys and return their sum.</summary>
    public static long LookupEntries(Dictionary<long, long> dict, int count)
    {
        long sum = 0;
        for (long i = 0; i < count; i++)
        {
            if (dict.TryGetValue(i, out long val))
                sum += val;
        }
        return sum;
    }
}
