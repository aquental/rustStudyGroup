use std::collections::HashMap;
use std::time::{Duration, Instant};

mod actix;
mod tokio;
mod veqdeque;

struct Cache<K, V> {
    store: HashMap<K, V>,
    expirations: HashMap<K, Instant>, // Tracks expiration times for keys
    ttl: Option<Duration>,            // Optional TTL for cache entries
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    /// Construct a new `Cache` with an optional TTL.
    ///
    /// If `ttl` is `None`, the cache will never expire entries.
    /// If `ttl` is `Some(Duration)`, all entries will expire after
    /// the given duration after they are inserted.
    ///
    /// The cache is initially empty.
    fn new(ttl: Option<Duration>) -> Self {
        Cache {
            store: HashMap::new(),
            expirations: HashMap::new(),
            ttl,
        }
    }

    /// Insert a key-value pair into the cache.
    ///
    /// If the cache has a TTL set, the key-value pair will expire after
    /// the given duration from the time it was inserted.
    ///
    /// If the key already exists in the cache, its value will be updated.
    ///
    fn put(&mut self, key: K, value: V) {
        self.store.insert(key.clone(), value);
        if let Some(ttl) = self.ttl {
            self.expirations.insert(key, Instant::now() + ttl);
        }
    }

    /// Retrieve a value from the cache.
    ///
    /// If the key is not present in the cache, `None` is returned.
    ///
    /// If the key is present, but has expired, it is removed from the cache
    /// and `None` is returned.
    ///
    /// Otherwise, the value associated with the key is returned.
    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(expiration) = self.expirations.get(key) {
            if Instant::now() > *expiration {
                self.store.remove(key);
                self.expirations.remove(key);
                return None;
            }
        }
        self.store.get(key)
    }

    /// Remove a key-value pair from the cache.
    ///
    /// If the key-value pair had a TTL, its expiration entry is also removed.
    ///
    fn remove(&mut self, key: &K) {
        self.store.remove(key);
        self.expirations.remove(key);
    }
}

/// Simple test of the cache. Puts a value in the cache, retrieves it, waits
/// two seconds, and then retrieves it again to verify that the value has been
/// removed after its TTL. Also demonstrates manual removal.
fn main() {
    let mut cache: Cache<String, i32> = Cache::new(Some(Duration::from_secs(1)));
    cache.put("key1".to_string(), 42);
    println!("Value: {:?}", cache.get(&"key1".to_string())); // Some(42)

    // Demonstrate manual removal
    cache.put("key2".to_string(), 100);
    println!("Value before removal: {:?}", cache.get(&"key2".to_string())); // Some(100)
    cache.remove(&"key2".to_string());
    println!("Value after removal: {:?}", cache.get(&"key2".to_string())); // None

    std::thread::sleep(Duration::from_secs(2));
    println!("Value after TTL: {:?}", cache.get(&"key1".to_string())); // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_put_get() {
        let mut cache: Cache<String, i32> = Cache::new(None);
        cache.put("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(&42));
    }

    #[test]
    fn test_cache_put_get_expired() {
        let mut cache: Cache<String, i32> = Cache::new(Some(Duration::from_millis(100)));
        cache.put("key1".to_string(), 42);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_put_update() {
        let mut cache: Cache<String, i32> = Cache::new(None);
        cache.put("key1".to_string(), 42);
        cache.put("key1".to_string(), 43);
        assert_eq!(cache.get(&"key1".to_string()), Some(&43));
    }

    #[test]
    fn test_cache_remove() {
        let mut cache: Cache<String, i32> = Cache::new(None);
        cache.put("key1".to_string(), 42);
        cache.remove(&"key1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), None);
    }
}
