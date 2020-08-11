use std::cmp::min;

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.len() == 0 {
        return 0;
    }
    let width = matrix[0].len();
    let mut height = vec![0;width];
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut max = 0;
    for row in matrix {
        for (index, cur) in row.iter().enumerate() {
            if *cur == '0' {
                height[index] = 0;
            } else {
                height[index] += 1;
            }
            let cur_height = height[index];
            while !stack.is_empty() {
                let top = stack[stack.len()-1];
                let top_height = top.0;
                let top_index = top.1;
                if top_height <= cur_height {
                    break;
                }
                stack.pop();
                let right = index as i32;
                let left: i32 = if stack.is_empty() {-1} else {stack[stack.len()-1].1 as i32};
                let cur_width = min((right - left - 1), top_height);
                let local = cur_width * cur_width;
                println!("{} {} {} {} {}", top_index, top_height, cur_width, left, right);
                if local > max {
                    max = local;
                }
            }
            stack.push((cur_height, index));
        }
        while !stack.is_empty() {
            let top = stack[stack.len()-1];
            let top_height = top.0;
            let top_index = top.1;
            stack.pop();
            let right = width as i32;
            let left: i32 = if stack.is_empty() {-1} else {stack[stack.len()-1].1 as i32};
            let cur_width = min((right - left - 1), top_height);
            let local = cur_width * cur_width;
            println!("{} {} {} {} {}", top_index, top_height, cur_width, left, right);
            if local > max {
                max = local;
            }
        }
        println!();
    }
    return max;
}