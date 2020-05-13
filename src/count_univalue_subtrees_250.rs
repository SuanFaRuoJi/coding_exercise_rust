use std::rc::Rc;
use std::cell::RefCell;
use crate::tree_node::TreeNode;

pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (total_count, _) = recurse(root);
    total_count
}

fn recurse(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<i32>){
    match node {
        None => (0, None),
        Some(node_val) => {
            let tree_node = (*node_val).borrow_mut();
            let mut uni_value = true;
            let (left_count, left_uni) = recurse(tree_node.left.clone());
            match left_uni {
                Some(uni) => {
                    uni_value = uni_value && (uni == tree_node.val);
                },
                None => {
                    uni_value = uni_value && (left_count == 0);
                }
            };
            let (right_count, right_uni) = recurse(tree_node.right.clone());
            match right_uni {
                Some(uni) => {
                    uni_value = uni_value && (uni == tree_node.val);
                },
                None => {
                    uni_value = uni_value && (right_count == 0);
                }
            };
            if uni_value {
                (left_count + right_count + 1, Some(tree_node.val))
            } else {
                (left_count + right_count, None)
            }
        }
    }
}