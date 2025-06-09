use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

struct AsyncCache<K, V> {
    store: HashMap<K, V>,
    expirations: HashMap<K, Instant>,
    ttl: Option<Duration>,
}

impl<K, V> AsyncCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    /// Construct a new `AsyncCache` with an optional TTL.
    ///
    /// If `ttl` is `None`, the cache will never expire entries.
    /// If `ttl` is `Some(Duration)`, all entries will expire after
    /// the given duration after they are inserted.
    ///
    /// The cache is initially empty.
    fn new(ttl: Option<Duration>) -> Self {
        AsyncCache {
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
    async fn put(&mut self, key: K, value: V) {
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
    async fn get(&mut self, key: &K) -> Option<&V> {
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
    async fn remove(&mut self, key: &K) {
        self.store.remove(key);
        self.expirations.remove(key);
    }
}

/// Simple test of the async cache. Puts a value in the cache, retrieves it, waits
/// two seconds, and then retrieves it again to verify that the value has been
/// removed after its TTL.
#[tokio::main]
async fn main() {
    let cache = Arc::new(Mutex::new(AsyncCache::new(Some(Duration::from_secs(1)))));

    {
        let mut cache = cache.lock().await;
        cache.put("key1".to_string(), 42).await;
        println!("Value: {:?}", cache.get(&"key1".to_string()).await); // Some(42)
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    {
        let mut cache = cache.lock().await;
        println!(
            "Value after TTL: {:?}",
            cache.get(&"key1".to_string()).await
        ); // None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_cache_put_get() {
        let cache = Arc::new(Mutex::new(AsyncCache::new(None)));
        cache.lock().await.put("key1".to_string(), 42).await;
        assert_eq!(cache.lock().await.get(&"key1".to_string()).await, Some(&42));
    }

    #[tokio::test]
    async fn test_async_cache_put_get_expired() {
        let cache = Arc::new(Mutex::new(AsyncCache::new(Some(Duration::from_millis(
            100,
        )))));
        cache.lock().await.put("key1".to_string(), 42).await;
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert_eq!(cache.lock().await.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_async_cache_put_update() {
        let cache = Arc::new(Mutex::new(AsyncCache::new(None)));
        cache.lock().await.put("key1".to_string(), 42).await;
        cache.lock().await.put("key1".to_string(), 43).await;
        assert_eq!(cache.lock().await.get(&"key1".to_string()).await, Some(&43));
    }

    #[tokio::test]
    async fn test_async_cache_remove() {
        let cache = Arc::new(Mutex::new(AsyncCache::new(None)));
        cache.lock().await.put("key1".to_string(), 42).await;
        cache.lock().await.remove(&"key1".to_string()).await;
        assert_eq!(cache.lock().await.get(&"key1".to_string()).await, None);
    }

    #[tokio::test]
    async fn test_async_cache_concurrent_access() {
        let cache = Arc::new(Mutex::new(AsyncCache::new(None)));
        let mut handles = vec![];

        for i in 0..10 {
            let cache_clone = cache.clone();
            handles.push(tokio::spawn(async move {
                cache_clone.lock().await.put(format!("key{}", i), i).await;
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        for i in 0..10 {
            assert_eq!(
                cache.lock().await.get(&format!("key{}", i)).await,
                Some(&(i as i32))
            );
        }
    }
}
