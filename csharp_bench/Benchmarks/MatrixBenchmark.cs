using BenchmarkDotNet.Attributes;
using CSharpBench.Core;

namespace CSharpBench.Benchmarks;

/// <summary>
/// Matrix multiply 512×512 — same size as Rust benchmark.
/// IterationCount reduced to keep total run time under 5 minutes.
/// </summary>
[MemoryDiagnoser]
[WarmupCount(3)]
[IterationCount(5)]
public class MatrixBenchmark
{
    private double[,] _a = new double[0, 0];
    private double[,] _b = new double[0, 0];
    private const int N = 512;

    [GlobalSetup]
    public void Setup()
    {
        _a = MatrixEngine.GenerateMatrix(42, N);
        _b = MatrixEngine.GenerateMatrix(43, N);
    }

    [Benchmark]
    public double[,] Multiply512x512() => MatrixEngine.Multiply(_a, _b, N);
}
