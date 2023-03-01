use std::collections::hash_map::DefaultHasher;
use std::collections::{hash_map, HashMap};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub struct ReverseHashMap<K: Eq + Hash, V> {
    map: HashMap<u64, V>,
    private_key: u64,
    key_type: PhantomData<K>,
}

#[derive(Hash, PartialEq)]
struct KeyToHash<K: Eq + Hash> {
    private_key: u64,
    map_key: K,
}

impl<K: Eq + Hash, V: PartialEq> ReverseHashMap<K, V> {
    pub fn new() -> ReverseHashMap<K, V> {
        ReverseHashMap {
            map: HashMap::new(),
            private_key: rand::random(),
            key_type: PhantomData,
        }
    }

    // inserts a value into the hashmap and returns the the previous value at that key if it exists.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key_to_be_hashed: KeyToHash<K> = KeyToHash {
            private_key: self.private_key,
            map_key: key,
        };
        let mut hasher: DefaultHasher = hash_map::DefaultHasher::new();
        key_to_be_hashed.hash(&mut hasher);
        let hashed_key: u64 = hasher.finish();
        return self.map.insert(hashed_key, value);
    }

    // Searches all possible values for hash and returns the first one that matches.
    pub fn find_key(&self, value: V, print_progress: bool) -> Option<u64> {
        for possible_key in 0..u64::MAX {
            let key_to_be_hashed: KeyToHash<u64> = KeyToHash {
                private_key: self.private_key,
                map_key: possible_key,
            };
            let mut hasher: DefaultHasher = hash_map::DefaultHasher::new();
            key_to_be_hashed.hash(&mut hasher);
            let hashed_key: u64 = hasher.finish();
            if self.map.get(&hashed_key) == Some(&value) {
                return Some(possible_key);
            }
            if print_progress && possible_key % 1000000 == 0 {
                println!(
                    "Progress: {} tested, {}% done",
                    possible_key,
                    possible_key / u64::MAX * 100
                );
            }
        }
        return None;
    }

    // Returns the value at the key if it exists.
    pub fn get(&self, key: u64) -> Option<&V> {
        return self.map.get(&key);
    }

    // Deletes the value at the key if it exists.
    pub fn delete(&mut self, key: u64) -> Option<V> {
        return self.map.remove(&key);
    }
}
