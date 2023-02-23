use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct SyncRedisHashMap<K, V> {
    pub map: Arc<Mutex<HashMap<K, V>>>,
}

#[allow(dead_code)]
impl<K, V> SyncRedisHashMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::<K, V>::new())),
        }
    }

    fn insert(&self, key: K, value: V) {
        let mut map = self.map.lock().unwrap();
        map.insert(key, value);
    }

    fn remove(&self, key: &K) {
        let mut map = self.map.lock().unwrap();
        map.remove(key);
    }

    fn get(&self, key: &K) -> Option<V> {
        let map = self.map.lock().unwrap();
        map.get(key).cloned()
    }

    fn len(&self) -> usize {
        let map = self.map.lock().unwrap();
        map.len()
    }
}
