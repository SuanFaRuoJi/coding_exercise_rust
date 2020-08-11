pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
    let mut prefix = Vec::new();
    prefix.push(0);
    let mut stack: Vec<usize> = Vec::new();
    let mut sum = 0;
    let mut min = -1;
    let mut left: usize = 0;
    for cur in a.iter() {
        sum += *cur;
        prefix.push(sum);
    }
    let mut index = 0;
    while index < prefix.len() {
        let cur = prefix[index];
        while !stack.is_empty() && cur < prefix[stack[stack.len()-1]] {
            stack.pop();
        }
        stack.push(index);
        while left < stack.len() && cur - prefix[stack[left]] >= k {
            let local = (index - stack[left]) as i32;
            if min == -1 || min >= local {
                min = local;
            }
            left += 1;
        }

        index += 1;
    }
    return min;
}