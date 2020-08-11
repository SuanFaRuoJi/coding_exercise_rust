pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right = citations.len();
    let mut sum= 0;
    for citation in citations {
        if citation >= right as i32 {
            sum += 1;
        }
    }
    if sum == right as i32 {
        return sum;
    }
    // This means right is not achieveable
    while left+1 < right {
        let mid = (left+right)/2;
        let mut local = 0;
        for citation in citations {
            if citation >= mid as i32 {
                local += 1;
            }
        }
        
    }
    return sum;
}