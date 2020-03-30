use std::convert::TryInto;

pub fn triangle_number(nums: Vec<i32>) -> i32 {
    let mut operate_on: Vec<i32> = Vec::new();
    operate_on.extend(nums.iter());
    operate_on.sort();
    let mut index = 0;
    let mut count: i32 = 0;
    for mark in operate_on.iter() {
        count += larger_than_mark(&operate_on[0..index], *mark);
        index += 1;
    }
    count
}

#[inline]
fn larger_than_mark(nums: &[i32], mark: i32) -> i32 {
    // println!("Length: {}, mark: {}", nums.len(), mark);
    if nums.len() < 2 {
        return 0
    }
    let mut left: usize = 0;
    let mut right = nums.len() - 1;
    let mut count: i32 = 0;
    while left < right {
        if nums[left] + nums[right] > mark {
            count += (right - left) as i32;
            right -= 1;
        }
        else {
            left += 1;
        }
    }
    count
}