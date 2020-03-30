pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
    let mut cur: Vec<i32> = Vec::new();
    let mut global = Vec::new();
    search(n, 2, &mut cur, &mut global);
    return global;
}

fn search(goal: i32, start: i32, cur: &mut Vec<i32>, global: &mut Vec<Vec<i32>>) {
    println!("{}: {}", goal, cur.len());
    if goal == 1 {
        if cur.len() > 1 {
            global.push(cur.clone());
        }
        return;
    }

    let mut i: i32 = start;
    while i * i <= goal {
        if goal % i == 0 {
            cur.push(i);
            search(goal / i, i, cur, global);
            cur.pop();
        }
        i += 1;
    }
    if cur.len() > 0 {
        cur.push(goal);
        global.push(cur.clone());
        cur.pop();
    }
}