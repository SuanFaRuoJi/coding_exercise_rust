pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>)
}

pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
    let arr: [i32;3] = weight_sum_and_depth(&NestedInteger::List(nested_list), 0);
    println!("[{}, {}, {}]", arr[0], arr[1], arr[2]);
    (arr[1] + 1) * arr[2] - arr[0]
}

fn weight_sum_and_depth(sub_list: & NestedInteger, depth: i32) -> [i32;3] {
    match sub_list {
        NestedInteger::Int(num) => [num * depth, depth, *num],
        NestedInteger::List(actual_list) => {
            let mut sum: i32 = 0;
            let mut max_depth: i32 = depth;
            let mut unweighted_sum: i32 = 0;
            for node in actual_list.iter() {
                let cur_result = weight_sum_and_depth(node, depth + 1);
                if cur_result[1] > max_depth {
                    max_depth = cur_result[1];
                }
                sum += cur_result[0];
                unweighted_sum += cur_result[2];
            }
            [sum, max_depth, unweighted_sum]
        }
    }
}