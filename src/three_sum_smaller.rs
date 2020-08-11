pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    nums.sort();
    let mut index: usize = 0;
    let mut sum = 0;
    while index < nums.len()-2 {
        let mut partial_target = target - nums[index];
        let mut left = index + 1;
        let mut right = nums.len() - 1;
        while left != right {
            println!("{}: {}, {}: {}", left, nums[left], right, nums[right]);
            let partial_sum = nums[left] + nums[right];
            if partial_sum < partial_target {
                sum += (right - left) as i32;
                left += 1;
            } else {
                right -= 1;
            }
        }
        index += 1;
    }
    return sum;
}