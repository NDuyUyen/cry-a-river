use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .and_then(|v| match v.partition_point(|&(t, _)| t <= timestamp) {
                0 => None,
                pp => Some(v[pp - 1].1.clone()),
            })
            .unwrap_or_default()
    }
}
