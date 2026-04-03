use std::collections::HashMap;

/// Insert `count` sequential key-value pairs into a new `HashMap`.
/// key = i, value = i * i  (u64).
pub fn insert_entries(count: usize) -> HashMap<u64, u64> {
    let mut map = HashMap::with_capacity(count);
    for i in 0..count as u64 {
        map.insert(i, i * i);
    }
    map
}

/// Look up all `count` keys and return their sum (prevents dead-code elimination).
pub fn lookup_entries(map: &HashMap<u64, u64>, count: usize) -> u64 {
    let mut sum = 0u64;
    for i in 0..count as u64 {
        sum = sum.wrapping_add(*map.get(&i).unwrap_or(&0));
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_lookup() {
        let map = insert_entries(100);
        assert_eq!(map.len(), 100);
        // key 5 -> value 25
        assert_eq!(*map.get(&5).unwrap(), 25);
    }

    #[test]
    fn lookup_sum() {
        let map = insert_entries(4); // 0,1,4,9 -> sum=14
        assert_eq!(lookup_entries(&map, 4), 14);
    }
}
