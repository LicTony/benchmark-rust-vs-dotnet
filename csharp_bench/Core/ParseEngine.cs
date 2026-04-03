using System.Text;

namespace CSharpBench.Core;

/// <summary>
/// String parsing engine: generate synthetic text, count words, transform to uppercase.
/// Equivalent to Rust's parsing.rs.
/// </summary>
public static class ParseEngine
{
    private static readonly string[] Words =
        ["the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog", "and", "cat"];

    /// <summary>Generate a synthetic text string of approximately <paramref name="sizeBytes"/> bytes.</summary>
    public static string GenerateText(int sizeBytes)
    {
        var sb = new StringBuilder(sizeBytes + 16);
        int i = 0;
        while (sb.Length < sizeBytes)
        {
            sb.Append(Words[i % Words.Length]);
            sb.Append(' ');
            i++;
        }
        return sb.ToString();
    }

    /// <summary>Count whitespace-delimited words — equivalent to split_ascii_whitespace in Rust.</summary>
    public static int CountWords(string text)
    {
        int count = 0;
        bool inWord = false;
        foreach (char ch in text)
        {
            if (char.IsWhiteSpace(ch))
            {
                inWord = false;
            }
            else if (!inWord)
            {
                inWord = true;
                count++;
            }
        }
        return count;
    }

    /// <summary>Convert text to uppercase using the invariant culture.</summary>
    public static string ToUpperCase(string text) => text.ToUpperInvariant();
}
