use std::borrow::Borrow;

pub fn verify_preorder(preorder: Vec<i32>) -> bool {
    let mut stack: Vec<i32> = vec![];
    let mut bound: Option<i32> = None;
    for cur in preorder {
        match bound {
            None => {},
            Some(val) => {
                if cur < val {
                    return false
                }
            }
        }
        while !stack.is_empty() && stack[stack.len()-1] < cur {
           bound = stack.pop();
        }
        stack.push(cur);
    }
    true
}