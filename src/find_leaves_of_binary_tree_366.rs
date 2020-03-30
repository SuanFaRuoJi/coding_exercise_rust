use crate::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    match root {
        Some(node) => {
            reverse_height(node, 0, &mut result);
        },
        None => {}
    }
    return result;
}

fn reverse_height(root: Rc<RefCell<TreeNode>>, current: usize, result: &mut Vec<Vec<i32>>) -> usize {
    if current >= result.len() {
        result.push(Vec::new());
    }
    let mut max: usize = 0;
    match &root.borrow().left {
        Some(l_node) => {
            let left_height = reverse_height(l_node.clone(), current + 1, result);
            if left_height > max {
                max = left_height;
            }

        },
        None => {}
    }
    match &root.borrow().right {
        Some(r_node) => {
            let right_height = reverse_height(r_node.clone(), current + 1, result);
            if right_height > max {
                max = right_height;
            }
        },
        None => {}
    }
    result[max].push(root.borrow().val);
    return max + 1;
}
