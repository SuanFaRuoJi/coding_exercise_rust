use std::rc::Rc;
use std::cell::RefCell;
use super::ds::TreeNode;
use std::borrow::{BorrowMut, Borrow};

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let arr: Vec<i32> = flatten(root);
    println!("{:?}", arr);
    return arr[k as usize];
}

fn flatten(head: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match head {
        None => vec![],
        Some(valBox) => {
            let node = (*valBox).borrow();
            let val: i32 = node.val;
            let mut left: Vec<i32> = flatten(node.left.clone());
            let mut right: Vec<i32> = flatten(node.right.clone());
            let mut result: Vec<i32> = Vec::new();
            result.append(left.borrow_mut());
            result.push(val);
            result.append(right.borrow_mut());
            result
        }
    }
}
