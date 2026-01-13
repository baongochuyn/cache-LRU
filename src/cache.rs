use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

/// Core trait for LRU cache behavior
pub trait CacheLRU<K, V> {
    fn on_get(&mut self, key: &K) -> Option<V>;
    fn on_put(&mut self, key: K, value: V) -> Option<V>;
    fn remove_least_recently_used(&mut self) -> Option<K>;
}

/// Internal structure storing value and access sequence number
#[derive(Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub sequence: u64,
}

#[allow(dead_code)]
pub struct Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub map: HashMap<K, CacheEntry<V>>,
    pub capacity: usize,
    pub sequence: u64,  // Global access sequence counter
    pub persist_path: Option<String>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    /// Create a new cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            capacity,
            sequence: 0,
            persist_path: None,
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        <Self as CacheLRU<K, V>>::on_put(self, key, value)
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        <Self as CacheLRU<K, V>>::on_get(self, key)
    }
}

impl<K, V> CacheLRU<K, V> for Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn on_get(&mut self, key: &K) -> Option<V> {
        if let Some(entry) = self.map.get_mut(key) {
            self.sequence = self.sequence + 1;
            entry.sequence = self.sequence;
            Some(entry.value.clone())
        } else {
            None
        }
    }

    fn on_put(&mut self, key: K, value: V) -> Option<V> {
        self.sequence = self.sequence + 1;
        let entry = CacheEntry {
            value,
            sequence: self.sequence,
        };
        let prev = self.map.insert(key.clone(), entry);
        if prev.is_none() && self.map.len() > self.capacity {
            self.remove_least_recently_used();
        }
        prev.map(|e| e.value)
    }

    fn remove_least_recently_used(&mut self) -> Option<K> {
        if let Some(lru_key) = self.map
            .iter()
            .min_by_key(|(_, entry)| entry.sequence)
            .map(|(k, _)| k.clone())
        {
            self.map.remove(&lru_key);
            Some(lru_key)
        } else {
            None
        }
    }
}

// Persistence methods 
impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + ToString + FromStr,
    V: Clone + ToString + FromStr,
    <K as FromStr>::Err: std::fmt::Debug,
    <V as FromStr>::Err: std::fmt::Debug,
{
    /// Create a new persistent cache with path
    pub fn new_persistent<P: AsRef<std::path::Path>>(capacity: usize, path: P) -> Self {
        Self {
            map: HashMap::new(),
            capacity,
            sequence: 0,
            persist_path: Some(path.as_ref().to_string_lossy().to_string()),
        }
    }

    /// Load persistent cache from file, store path 
    pub fn load_persistent<P: AsRef<std::path::Path>>(capacity: usize, path: P) -> std::io::Result<Self> {
        let mut cache = Self::new_persistent(capacity, &path);
        let file_path = path.as_ref();
        if file_path.exists() {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((kv, seq_str)) = line.split_once('\t') {
                    if let Some((key, value)) = kv.split_once('=') {
                        let k = K::from_str(key).expect("Failed to parse key");
                        let v = V::from_str(value).expect("Failed to parse value");
                        let seq = u64::from_str(seq_str).expect("Failed to parse sequence");
                        cache.sequence = cache.sequence.max(seq);
                        cache.map.insert(k, CacheEntry { value: v, sequence: seq });
                    }
                }
            }
        }
        Ok(cache)
    }

    /// Persist cache to file (overwrite if exists, for eligible K,V)
    pub fn persist(&self) -> std::io::Result<()> {
        let path = self.persist_path.as_ref().expect("No persist_path set");
        let _ = std::fs::remove_file(path);
        let mut file = File::create(path)?;

        let mut entries: Vec<_> = self.map.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.sequence);
        for (key, entry) in entries {
            let line = format!("{}={}	{}\n", key.to_string(), entry.value.to_string(), entry.sequence);
            file.write_all(line.as_bytes())?;
        }
        Ok(())
    }
}



