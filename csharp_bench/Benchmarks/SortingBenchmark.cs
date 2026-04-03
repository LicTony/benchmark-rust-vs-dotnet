using BenchmarkDotNet.Attributes;
using CSharpBench.Core;

namespace CSharpBench.Benchmarks;

[MemoryDiagnoser]
public class SortingBenchmark
{
    private int[] _data = [];

    /// <summary>Generate the source array once; Sort() clones it internally.</summary>
    [GlobalSetup]
    public void Setup() => _data = SortEngine.GenerateArray(42, 1_000_000);

    [Benchmark]
    public int[] Sort1M() => SortEngine.Sort(_data);
}
