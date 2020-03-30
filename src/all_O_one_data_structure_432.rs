use std::collections::{HashMap, HashSet};

struct AllOne {
    buckets: Vec<HashSet<String>>,
    reference: HashMap<String, usize>,
    left: usize
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    /** Initialize your data structure here. */
    fn new() -> Self {
        AllOne {
            buckets: vec![],
            reference: HashMap::new(),
            left: 0
        }
    }

    /** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
    fn inc(&mut self, key: String) {
    }

    /** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
    fn dec(&mut self, key: String) {

    }

    /** Returns one of the keys with maximal value. */
    fn get_max_key(&self) -> String {
        String::from("")
    }

    /** Returns one of the keys with Minimal value. */
    fn get_min_key(&self) -> String {
        String::from("")
    }
}