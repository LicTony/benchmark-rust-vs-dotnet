using BenchmarkDotNet.Attributes;
using CSharpBench.Core;

namespace CSharpBench.Benchmarks;

[MemoryDiagnoser]
public class HashMapBenchmark
{
    private Dictionary<long, long> _map = [];
    private const int Count = 1_000_000;

    [GlobalSetup]
    public void Setup() => _map = HashMapEngine.InsertEntries(Count);

    /// <summary>Measures dictionary construction cost (insert 1M entries).</summary>
    [Benchmark]
    public Dictionary<long, long> Insert1M() => HashMapEngine.InsertEntries(Count);

    /// <summary>Measures lookup cost against a pre-populated dictionary.</summary>
    [Benchmark]
    public long Lookup1M() => HashMapEngine.LookupEntries(_map, Count);
}
