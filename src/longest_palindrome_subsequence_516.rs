pub fn longest_palindrome_subseq(s: String) -> i32 {
    let mut indices: Vec<i32> = Vec::new();
    for char in s.chars() {
        indices.push(char as i32);
    }
    let length: usize = s.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0;length];length];
    let mut index = 0;
    while index < length {
        let mut left: usize = 0;
        let mut right: usize = index;
        while left < length && right < length {
            if indices[left] == indices[right] {
                if left+2 <= right {
                    dp[left][right] = dp[left+1][right-1] + 2;
                } else {
                    if left == right {
                        dp[left][right] = 1;
                    } else {
                        dp[left][right] = 2;
                    }
                }
            } else {
                dp[left][right] = i32::max(dp[left][right-1], dp[left+1][right]);
            }
            left += 1;
            right += 1;
        }
        index += 1;
    }
    dp[0][length-1]
}

fn backtrace(string: &Vec<i32>, left: usize, right: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if left == right {
        return 1;
    }
    if dp[left][right] != 0 {
        return dp[left][right];
    }
    if string[left] == string[right] {
        if left+1 > right-1 {
            dp[left][right] = 2;
            return 2;
        } else {
            let result = backtrace(string, left+1, right-1, dp);
            dp[left][right] = result + 2;
            return result + 2;
        }
    } else {
        let l_result = backtrace(string, left, right-1, dp);
        let r_result = backtrace(string, left+1, right, dp);
        let mut result = l_result;
        if r_result > l_result {
            result = r_result;
        }
        return result;
    }
}