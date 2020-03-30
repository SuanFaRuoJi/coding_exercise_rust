pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
    let mut occurrence_1: Vec<i32> = Vec::new();
    let mut occurrence_2: Vec<i32> = Vec::new();
    let mut index = 0;

    for word in words.iter() {
        if word1.eq(word) {
            occurrence_1.push(index);
        }
        if word2.eq(word) {
            occurrence_2.push(index);
        }
        index += 1;
    }

    if word1.eq(&word2) {
        let mut prev: i32 = -1;
        let mut min_dist = -1;
        for index in &occurrence_1 {
            if prev != -1 {
                let cur_dist = *index - prev;
                if min_dist == -1 || cur_dist < min_dist {
                    min_dist = cur_dist;
                }
            }
            prev = *index;
        }
        return min_dist;
    }

    let mut min_dist: i32 = -1;
    let mut iter_1 = 0;
    let mut iter_2 = 0;
    while iter_1 < occurrence_1.len() && iter_2 < occurrence_2.len() {
        let val_1 = occurrence_1[iter_1];
        let val_2 = occurrence_2[iter_2];
        // println!("O1: {}, O2: {}", val_1, val_2);
        if val_1 > val_2 {
            let cur_dist: i32 = val_1 - val_2;
            if min_dist == -1 || cur_dist < min_dist {
                min_dist = cur_dist;
            }
            iter_2 += 1;
        } else {
            let cur_dist: i32 = val_2 - val_1;
            if min_dist == -1 || cur_dist < min_dist {
                min_dist = cur_dist;
            }
            iter_1 += 1;
        }
    }
    min_dist
}