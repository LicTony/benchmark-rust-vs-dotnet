const WORDS: &[&str] = &[
    "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog", "and", "cat",
];

/// Generate a synthetic text string of approximately `size_bytes` bytes.
/// Deterministic: always produces the same output for the same input size.
pub fn generate_text(size_bytes: usize) -> String {
    let mut text = String::with_capacity(size_bytes + 16);
    let mut i = 0usize;
    while text.len() < size_bytes {
        text.push_str(WORDS[i % WORDS.len()]);
        text.push(' ');
        i += 1;
    }
    text
}

/// Count whitespace-delimited words in `text`.
pub fn count_words(text: &str) -> usize {
    text.split_ascii_whitespace().count()
}

/// Convert the entire text to ASCII uppercase.
pub fn to_uppercase(text: &str) -> String {
    text.to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_small() {
        assert_eq!(count_words("hello world foo"), 3);
    }

    #[test]
    fn generate_fills_target_size() {
        let text = generate_text(1024);
        assert!(text.len() >= 1024);
    }

    #[test]
    fn uppercase_roundtrip() {
        let s = "hello";
        assert_eq!(to_uppercase(s), "HELLO");
    }
}
