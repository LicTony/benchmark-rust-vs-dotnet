using BenchmarkDotNet.Attributes;
using CSharpBench.Core;

namespace CSharpBench.Benchmarks;

[MemoryDiagnoser]
public class PrimeBenchmark
{
    [Benchmark]
    public long SumPrimes10M() => PrimeEngine.SumPrimesSieve(10_000_000);
}
