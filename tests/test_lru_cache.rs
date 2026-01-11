use cache::cache::*;

#[test]
fn test_lru_insert_and_eviction() {
    let mut cache = Cache::<String, String>::new(3);
    assert!(cache.put("A".to_string(), "value_a".to_string()).is_none());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());
    cache.put("D".to_string(), "value_d".to_string());
    assert_eq!(cache.get(&"A".to_string()), None);
    assert_eq!(cache.get(&"B".to_string()), Some("value_b".to_string()));
    assert_eq!(cache.get(&"C".to_string()), Some("value_c".to_string()));
    assert_eq!(cache.get(&"D".to_string()), Some("value_d".to_string()));
}

#[test]
fn test_lru_get_existing_and_nonexisting() {
    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    assert_eq!(cache.get(&"A".to_string()), Some("value_a".to_string()));
    assert_eq!(cache.get(&"B".to_string()), Some("value_b".to_string()));
    assert_eq!(cache.get(&"X".to_string()), None);
}

#[test]
fn test_lru_update_value() {
    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".to_string(), "value_a".to_string());
    let prev = cache.put("A".to_string(), "value_A".to_string());
    assert_eq!(prev, Some("value_a".to_string()));
    assert_eq!(cache.get(&"A".to_string()), Some("value_A".to_string()));
}

#[test]
fn test_lru_access_order() {
    let mut cache = Cache::<String, String>::new(3);
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());
    cache.get(&"B".to_string());
    cache.get(&"C".to_string());
    cache.put("D".to_string(), "value_d".to_string());
    assert_eq!(cache.get(&"A".to_string()), None);
    assert_eq!(cache.get(&"B".to_string()), Some("value_b".to_string()));
    assert_eq!(cache.get(&"C".to_string()), Some("value_c".to_string()));
    assert_eq!(cache.get(&"D".to_string()), Some("value_d".to_string()));
}

#[test]
fn test_linear_search_vector() {
    let vector = vec![("a", "value_a"), ("b", "value_b"), ("c", "value_c")];
    let found = vector.iter().find(|elt| elt.0 == "c");
    assert_eq!(found, Some(&("c", "value_c")));
    let not_found = vector.iter().find(|elt| elt.0 == "x");
    assert_eq!(not_found, None);
}

#[test]
fn test_hashmap_search() {
    let vector = vec![("a", "value_a"), ("b", "value_b"), ("c", "value_c")];
    let map: std::collections::HashMap<&str, &str> = vector.into_iter().collect();
    assert_eq!(map.get(&"a"), Some(&"value_a"));
    assert_eq!(map.get(&"c"), Some(&"value_c"));
    assert_eq!(map.get(&"x"), None);
}

#[test]
fn test_cache_get_performance_and_correctness() {
    let mut cache = Cache::<String, String>::new(100);
    for i in 0..100 {
        cache.put(i.to_string(), format!("value_{}", i));
    }
    assert_eq!(cache.get(&"0".to_string()), Some("value_0".to_string()));
    assert_eq!(cache.get(&"99".to_string()), Some("value_99".to_string()));
    assert_eq!(cache.get(&"100".to_string()), None);
}

#[test]
fn test_lru_capacity_overflow() {
    let mut cache = Cache::<i32, String>::new(2);
    cache.put(1, "one".to_string());
    cache.put(2, "two".to_string());
    cache.put(3, "three".to_string());
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some("two".to_string()));
    assert_eq!(cache.get(&3), Some("three".to_string()));
}

#[test]
fn test_lru_multiple_evictions() {
    let mut cache = Cache::<char, i32>::new(3);
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('c', 3);
    cache.put('d', 4);
    cache.put('e', 5);
    cache.put('f', 6);
    assert_eq!(cache.get(&'d'), Some(4));
    assert_eq!(cache.get(&'e'), Some(5));
    assert_eq!(cache.get(&'f'), Some(6));
    assert_eq!(cache.get(&'a'), None);
    assert_eq!(cache.get(&'b'), None);
    assert_eq!(cache.get(&'c'), None);
}

#[test]
fn test_cache_new_persistent_and_persist() {
    let test_file = "test_persistent.txt";
    let _ = std::fs::remove_file(test_file);
    let mut cache = Cache::<String, String>::new_persistent(10, test_file);
    cache.put("persistent_key".to_string(), "persistent_value".to_string());
    assert!(cache.persist().is_ok());
    let mut loaded = Cache::<String, String>::load_persistent(10, test_file)
        .expect("Failed to load persistent cache");
    assert_eq!(loaded.get(&"persistent_key".to_string()), Some("persistent_value".to_string()));
    let _ = std::fs::remove_file(test_file);
}

#[test]
fn test_cache_persist_overwrites_file() {
    let test_file = "test_overwrite.txt";
    let _ = std::fs::remove_file(test_file);
    let mut cache1 = Cache::<String, String>::new_persistent(10, test_file);
    cache1.put("k1".to_string(), "v1".to_string());
    cache1.persist().unwrap();
    let mut cache2 = Cache::<String, String>::new_persistent(10, test_file);
    cache2.put("k2".to_string(), "v2".to_string());
    cache2.persist().unwrap();
    let mut loaded = Cache::<String, String>::load_persistent(10, test_file).unwrap();
    assert_eq!(loaded.get(&"k1".to_string()), None);
    assert_eq!(loaded.get(&"k2".to_string()), Some("v2".to_string()));
    let _ = std::fs::remove_file(test_file);
}
