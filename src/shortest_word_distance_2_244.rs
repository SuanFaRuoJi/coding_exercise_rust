use std::collections::HashMap;
use std::borrow::BorrowMut;

pub struct WordDistance {
    occurrences: HashMap<String, Vec<usize>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDistance {

    pub fn new(words: Vec<String>) -> Self {
        let mut result = WordDistance {
            occurrences: HashMap::new()
        };
        let mut index: usize = 0;
        for word in &words {
            if !result.occurrences.contains_key(word) {
                result.occurrences.insert(word.clone(), Vec::new());
            }
            match result.occurrences.get_mut(word) {
                Some(vec) => vec.push(index),
                None => {}
            }
            index += 1;
        }
        result
    }

    pub fn shortest(&self, word1: String, word2: String) -> i32 {
        let mut index_1: usize = 0;
        let mut index_2: usize = 0;
        let occurrence_1: &Vec<usize> = self.occurrences.get(&word1).unwrap();
        let occurrence_2: &Vec<usize> = self.occurrences.get(&word2).unwrap();
        let mut min: i32 = -1;
        while index_1 != occurrence_1.len() && index_2 != occurrence_2.len() {
            let cur_1: usize = occurrence_1[index_1];
            let cur_2: usize = occurrence_2[index_2];
            if cur_1 > cur_2 {
                let cur = (cur_1 - cur_2) as i32;
                if cur < min || min == -1 {
                    min = cur;
                }
                index_2 += 1;
            } else {
                let cur = (cur_2 - cur_1) as i32;
                if cur < min || min == -1 {
                    min = cur;
                }
                index_1 += 1;
            }
        }
        min
    }
}
