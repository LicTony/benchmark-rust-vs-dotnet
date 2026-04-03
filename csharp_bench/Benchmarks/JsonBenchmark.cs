using BenchmarkDotNet.Attributes;
using CSharpBench.Core;

namespace CSharpBench.Benchmarks;

[MemoryDiagnoser]
public class JsonBenchmark
{
    private List<Record> _records = [];
    private string _json = string.Empty;

    [GlobalSetup]
    public void Setup()
    {
        _records = JsonEngine.GenerateRecords(100_000);
        _json = JsonEngine.Serialize(_records);
    }

    [Benchmark]
    public string Serialize() => JsonEngine.Serialize(_records);

    [Benchmark]
    public List<Record> Deserialize() => JsonEngine.Deserialize(_json);
}
