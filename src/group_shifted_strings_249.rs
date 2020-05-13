use std::collections::HashMap;

pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut store: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
    for string in strings {
        let identifier = generate_identifier(&string);
        store.entry(identifier).or_insert(vec![]).push(string.clone());
    }
    let mut result: Vec<Vec<String>> = Vec::new();
    for groups in store {
        result.push(groups.1);
    }
    result
}

fn generate_identifier(raw: &str) -> Vec<i32> {
    if raw.len() == 1 {
        return vec![];
    }
    let mut result: Vec<i32> = Vec::new();
    let mut init: bool = true;
    let mut prev_char: char = ' ';
    for cur_char in raw.chars() {
        if init {
            init = false;
        } else {
            let mut diff = cur_char as i32 - prev_char as i32;
            if diff < 0 {
                diff += 26;
            }
            result.push(diff);
        }
        prev_char = cur_char;
    }
    println!("{}: {:?}", raw, result.clone());
    result
}
