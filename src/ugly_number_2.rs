use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

pub fn nth_ugly_number(n: i32) -> i32 {
    let mut pq: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut used: HashSet<i64> = HashSet::new();
    pq.push(Reverse(1));
    let mut index = 1;
    let mut answer = 0;
    while index <= n {
        match pq.pop() {
            Some(rev) => {
                answer = rev.0;
            },
            _ => {}
        }
        let mut to_add = answer * 2;
        if !used.contains(&to_add) {
            used.insert(to_add);
            pq.push(Reverse(to_add));
        }

        to_add = answer * 3;
        if !used.contains(&to_add) {
            used.insert(to_add);
            pq.push(Reverse(to_add));
        }

        to_add = answer * 5;
        if !used.contains(&to_add) {
            used.insert(to_add);
            pq.push(Reverse(to_add));
        }
        index += 1;
    }
    return answer as i32;
}