use actix::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheActor<K, V> {
    store: HashMap<K, V>,
    expirations: HashMap<K, Instant>,
    ttl: Option<Duration>,
}

impl<K, V> CacheActor<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + 'static,
    V: Clone + Send + 'static,
{
    fn new(ttl: Option<Duration>) -> Self {
        CacheActor {
            store: HashMap::new(),
            expirations: HashMap::new(),
            ttl,
        }
    }
}

impl Actor for CacheActor<String, i32> {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Option<i32>")]
struct Get(String);

#[derive(Message)]
#[rtype(result = "()")]
struct Put(String, i32);

#[derive(Message)]
#[rtype(result = "()")]
struct Remove(String);

impl Handler<Get> for CacheActor<String, i32> {
    type Result = Option<i32>;

    /// Handle a Get message by looking up the key in the store.
    ///
    /// If the key is not present in the cache, `None` is returned.
    ///
    /// If the key is present, but has expired, it is removed from the cache
    /// and `None` is returned.
    ///
    /// Otherwise, the value associated with the key is returned.
    fn handle(&mut self, msg: Get, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(expiration) = self.expirations.get(&msg.0) {
            if Instant::now() > *expiration {
                self.store.remove(&msg.0);
                self.expirations.remove(&msg.0);
                return None;
            }
        }
        self.store.get(&msg.0).cloned()
    }
}

impl Handler<Put> for CacheActor<String, i32> {
    type Result = ();

    /// Handle a Put message by inserting a key-value pair into the cache.
    ///
    /// If the cache has a TTL set, the key-value pair will expire after
    /// the given duration from the time it was inserted.
    ///
    /// If the key already exists in the cache, its value will be updated.
    fn handle(&mut self, msg: Put, _ctx: &mut Self::Context) -> Self::Result {
        self.store.insert(msg.0.clone(), msg.1);
        if let Some(ttl) = self.ttl {
            self.expirations.insert(msg.0, Instant::now() + ttl);
        }
    }
}

impl Handler<Remove> for CacheActor<String, i32> {
    type Result = ();

    /// Handle a Remove message by removing a key-value pair from the cache.
    fn handle(&mut self, msg: Remove, _ctx: &mut Self::Context) -> Self::Result {
        self.store.remove(&msg.0);
        self.expirations.remove(&msg.0);
    }
}

/// Simple test of the cache. Puts a value in the cache, retrieves it, waits
/// two seconds, and then retrieves it again to verify that the value has been
/// removed after its TTL.
#[actix::main]
async fn main() {
    let cache_addr = CacheActor::new(Some(Duration::from_secs(1))).start();

    cache_addr.do_send(Put("key1".to_string(), 42));
    let value = cache_addr.send(Get("key1".to_string())).await.unwrap();
    println!("Value: {:?}", value); // Some(42)

    tokio::time::sleep(Duration::from_secs(2)).await;
    let value = cache_addr.send(Get("key1".to_string())).await.unwrap();
    println!("Value after TTL: {:?}", value); // None
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the cache actor can put and get a value.
    #[actix::test]
    async fn test_cache_actor_put_get() {
        let cache_addr = CacheActor::new(None).start();
        cache_addr.do_send(Put("key1".to_string(), 42));
        let res = cache_addr.send(Get("key1".to_string())).await.unwrap();
        assert_eq!(res, Some(42));
    }

    /// Test that the cache actor can put and get a value, but that the value
    /// expires after its TTL.
    #[actix::test]
    async fn test_cache_actor_put_get_expired() {
        let cache_addr = CacheActor::new(Some(Duration::from_millis(100))).start();
        cache_addr.do_send(Put("key1".to_string(), 42));
        tokio::time::sleep(Duration::from_millis(200)).await;
        let res = cache_addr.send(Get("key1".to_string())).await.unwrap();
        assert_eq!(res, None);
    }

    /// Test that the cache actor can put and update a value.ß
    #[actix::test]
    async fn test_cache_actor_put_update() {
        let cache_addr = CacheActor::new(None).start();
        cache_addr.do_send(Put("key1".to_string(), 42));
        cache_addr.do_send(Put("key1".to_string(), 43));
        let res = cache_addr.send(Get("key1".to_string())).await.unwrap();
        assert_eq!(res, Some(43));
    }

/// Test that the cache actor can remove a value.
///
/// This test inserts a key-value pair into the cache, then removes it, 
/// and verifies that attempting to retrieve the key returns `None`.

    #[actix::test]
    async fn test_cache_actor_remove() {
        let cache_addr = CacheActor::new(None).start();
        cache_addr.do_send(Put("key1".to_string(), 42));
        cache_addr.do_send(Remove("key1".to_string()));
        let res = cache_addr.send(Get("key1".to_string())).await.unwrap();
        assert_eq!(res, None);
    }
}
