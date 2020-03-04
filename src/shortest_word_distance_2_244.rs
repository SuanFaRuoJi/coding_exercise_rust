use std::collections::HashMap;

struct WordDistance {
    distance: HashMap<String, HashMap<String, i32>>,
    latest: HashMap<String, i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDistance {

    fn new(words: Vec<String>) -> Self {
        let mut object: WordDistance = WordDistance {
          distance: HashMap::new(),
            latest: HashMap::new()
        };
        let counter = 0;
        for word in words {
            for (key, index) in object.latest.iter() {
                let cur_short = object.shortest(key.clone(), word.clone());
                let cur_dist = counter - index;
                if cur_short == -1 || cur_dist < cur_short {
                    match object.distance.get_mut(key) {
                        None => {
                            let to_insert = HashMap::new();
                            to_insert.insert(word.clone(), cur_dist);
                            object.distance.insert(key.clone(), to_insert);
                        },
                        Some(dists) => dists.insert(word.clone(), cur_dist)
                    }
                }
            }
            object.latest.insert(word.clone(), counter);
            counter += 1;
        }
        object
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        if word1 == word2 {
            return 0;
        }
        let dist1 = match self.distance.get(&word1) {
            None => -1,
            Some(dists) => match dists.get(&word2) {
                None => -1,
                Some(dist) => dist
            }
        };
        let dist2 = match self.distance.get(&word2) {
            None => -1,
            Some(dists) => match dists.get(&word1) {
                None => -1,
                Some(dist) => dist
            }
        };
        if dist1 == -1  {
            return dist2;
        }
        if dist2 == -1 {
            return dist1;
        }
        return dist1.min(dist2);
    }
}

/**
 * Your WordDistance object will be instantiated and called as such:
 * let obj = WordDistance::new(words);
 * let ret_1: i32 = obj.shortest(word1, word2);
 */