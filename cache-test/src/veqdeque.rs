use std::collections::VecDeque;
use std::time::{Duration, Instant};

struct VecDequeCache<K, V> {
    queue: VecDeque<(K, V, Option<Instant>)>, // (key, value, expiration)
    capacity: usize,                          // Max entries
    ttl: Option<Duration>,
}

impl<K, V> VecDequeCache<K, V>
where
    K: Eq + Clone,
    V: Clone,
{
    fn new(capacity: usize, ttl: Option<Duration>) -> Self {
        VecDequeCache {
            queue: VecDeque::with_capacity(capacity),
            capacity,
            ttl,
        }
    }

    /// Insert a key-value pair into the cache.
    ///
    /// If the cache has a TTL set, the key-value pair will expire after
    /// the given duration from the time it was inserted.
    ///
    /// If the cache is at capacity, the oldest entry (front of the queue)
    /// will be evicted before inserting the new key-value pair.
    fn put(&mut self, key: K, value: V) {
        // Remove old entry if exists
        self.remove(&key);
        // Evict oldest if at capacity
        if self.queue.len() >= self.capacity {
            self.queue.pop_front();
        }
        // Add new entry
        let expiration = self.ttl.map(|ttl| Instant::now() + ttl);
        self.queue.push_back((key, value, expiration));
    }

    /// Retrieve a value from the cache by key.
    ///
    /// If the key is not present in the cache, `None` is returned.
    ///
    /// If the key is present but has expired, it is removed from the cache
    /// and `None` is returned.
    ///
    /// Otherwise, the value associated with the key is returned and the entry
    /// is moved to the back of the queue to maintain the Least Recently Used (LRU) order.

    fn get(&mut self, key: &K) -> Option<&V> {
        // Find entry
        let index = self.queue.iter().position(|(k, _, _)| k == key)?;
        let (k, v, exp) = self.queue.remove(index).unwrap();
        // Check TTL
        if exp.map_or(false, |e| Instant::now() > e) {
            return None;
        }
        // Reinsert for LRU
        self.queue.push_back((k, v, exp));
        self.queue.back().map(|(_, v, _)| v)
    }

    /// Remove a key-value pair from the cache.
    ///
    /// If the key is present in the cache, the associated entry is removed.
    /// If the key is not present, the operation has no effect.

    fn remove(&mut self, key: &K) {
        if let Some(index) = self.queue.iter().position(|(k, _, _)| k == key) {
            self.queue.remove(index);
        }
    }
}

/// Demonstrates the usage of `VecDequeCache` by creating a cache with a capacity of 2 and a TTL of 1 second.
/// Inserts three key-value pairs and retrieves them, showcasing the LRU eviction policy and TTL expiration.

fn main() {
    let mut cache: VecDequeCache<String, i32> = VecDequeCache::new(2, Some(Duration::from_secs(1)));
    cache.put("key1".to_string(), 42);
    cache.put("key2".to_string(), 43);
    println!("Value: {:?}", cache.get(&"key1".to_string())); // Some(42)
    cache.put("key3".to_string(), 44); // Evicts key2
    println!("Value: {:?}", cache.get(&"key2".to_string())); // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veqdeque_new() {
        let veqdeque: VecDequeCache<String, i32> = VecDequeCache::new(10, None);
        assert_eq!(veqdeque.capacity, 10);
        assert_eq!(veqdeque.ttl, None);
    }

    #[test]
    fn test_veqdeque_put() {
        let mut veqdeque = VecDequeCache::new(10, None);
        veqdeque.put("key1".to_string(), 42);
        assert_eq!(veqdeque.queue.len(), 1);
        assert_eq!(veqdeque.queue.front().unwrap().0, "key1");
        assert_eq!(veqdeque.queue.front().unwrap().1, 42);
    }

    #[test]
    fn test_veqdeque_get() {
        let mut veqdeque = VecDequeCache::new(10, None);
        veqdeque.put("key1".to_string(), 42);
        assert_eq!(veqdeque.get(&"key1".to_string()), Some(&42));
    }

    #[test]
    fn test_veqdeque_remove() {
        let mut veqdeque = VecDequeCache::new(10, None);
        veqdeque.put("key1".to_string(), 42);
        veqdeque.remove(&"key1".to_string());
        assert_eq!(veqdeque.queue.len(), 0);
    }

    #[test]
    fn test_veqdeque_ttl() {
        let mut veqdeque = VecDequeCache::new(10, Some(Duration::from_millis(100)));
        veqdeque.put("key1".to_string(), 42);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(veqdeque.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_veqdeque_capacity() {
        let mut veqdeque = VecDequeCache::new(2, None);
        veqdeque.put("key1".to_string(), 42);
        veqdeque.put("key2".to_string(), 43);
        veqdeque.put("key3".to_string(), 44);
        assert_eq!(veqdeque.queue.len(), 2);
        assert_eq!(veqdeque.queue.front().unwrap().0, "key2");
        assert_eq!(veqdeque.queue.front().unwrap().1, 43);
    }
}
