using BenchmarkDotNet.Attributes;
using CSharpBench.Core;

namespace CSharpBench.Benchmarks;

/// <summary>
/// String parsing on a 50 MB synthetic text — same size as Rust benchmark.
/// IterationCount reduced because each iteration processes ~50 MB.
/// </summary>
[MemoryDiagnoser]
[WarmupCount(3)]
[IterationCount(5)]
public class ParsingBenchmark
{
    private string _text = string.Empty;

    [GlobalSetup]
    public void Setup() => _text = ParseEngine.GenerateText(50 * 1024 * 1024);

    [Benchmark]
    public int CountWords() => ParseEngine.CountWords(_text);

    [Benchmark]
    public string ToUpperCase() => ParseEngine.ToUpperCase(_text);
}
