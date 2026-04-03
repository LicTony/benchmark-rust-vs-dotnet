use serde::{Deserialize, Serialize};

/// A realistic record with mixed-type fields for JSON benchmarking.
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub name: String,
    pub value: f64,
    pub active: bool,
    pub tags: Vec<String>,
}

/// Generate `count` deterministic records (no RNG needed — derived from index).
pub fn generate_records(count: usize) -> Vec<Record> {
    (0..count)
        .map(|i| Record {
            id: i as u64,
            name: format!("record_{i}"),
            value: i as f64 * std::f64::consts::PI,
            active: i % 2 == 0,
            tags: vec![format!("tag_{}", i % 10), format!("cat_{}", i % 5)],
        })
        .collect()
}

/// Serialize records to a JSON string.
pub fn serialize(records: &[Record]) -> String {
    serde_json::to_string(records).expect("serialization must succeed")
}

/// Deserialize a JSON string into a `Vec<Record>`.
pub fn deserialize(json: &str) -> Vec<Record> {
    serde_json::from_str(json).expect("deserialization must succeed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let records = generate_records(10);
        let json = serialize(&records);
        let back = deserialize(&json);
        assert_eq!(back.len(), 10);
        assert_eq!(back[0].id, 0);
        assert_eq!(back[9].id, 9);
    }
}
