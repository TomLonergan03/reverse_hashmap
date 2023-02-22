pub struct ReverseHashMap<K, V> {
    map: HashMap<K, V>,
    private_key: u64,
}

impl ReverseHashMap {
    pub fn new() -> ReverseHashMap {
        ReverseHashMap {
            map: HashMap::new(),
            private_key: rand::thread_rng().gen(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hashed_key = calculate_hash(&key, &mut self.private_key);
        self.map.insert(hashed_key, value)
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.map.get(&key)
    }
}
