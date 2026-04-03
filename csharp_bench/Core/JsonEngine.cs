using System.Text.Json;

namespace CSharpBench.Core;

/// <summary>A realistic mixed-type record used for JSON benchmarking.</summary>
public sealed class Record
{
    public long Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public double Value { get; set; }
    public bool Active { get; set; }
    public List<string> Tags { get; set; } = [];
}

/// <summary>
/// JSON operations using System.Text.Json — equivalent to serde_json in Rust.
/// </summary>
public static class JsonEngine
{
    private static readonly JsonSerializerOptions _options = new()
    {
        // Match default serde_json behaviour (camelCase not required here)
    };

    public static List<Record> GenerateRecords(int count)
    {
        var records = new List<Record>(count);
        for (int i = 0; i < count; i++)
        {
            records.Add(new Record
            {
                Id = i,
                Name = $"record_{i}",
                Value = i * Math.PI,
                Active = i % 2 == 0,
                Tags = [$"tag_{i % 10}", $"cat_{i % 5}"]
            });
        }
        return records;
    }

    public static string Serialize(List<Record> records) =>
        JsonSerializer.Serialize(records, _options);

    public static List<Record> Deserialize(string json) =>
        JsonSerializer.Deserialize<List<Record>>(json, _options)!;
}
