
// https://www.geeksforgeeks.org/problems/maximum-path-sum/1
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use super::TreeNode;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    let mut max_sum = i32::MIN;
    max_sum_helper(&root, &mut max_sum);

    max_sum
}

pub fn max_sum_helper(root: &Option<Rc<RefCell<TreeNode>>>, acc: &mut i32) -> i32 {
    
    match root {
        None => 0,
        Some(val) => {
            let root_val = val.as_ref().borrow();

            // max left path sum
            let left_sum = max_sum_helper(&root_val.left, acc).max(0);

            // max right path sum
            let right_sum = max_sum_helper(&root_val.right, acc).max(0);

            // sum of the paths
            let mut total_sum = root_val.val + left_sum + right_sum;

            // I have to take into account the total sum of root, left child and right child,
            // as it is possible that this sum is greater than the max path sum of going 
            // deep donw the tree
            *acc = *acc.max(&mut total_sum); 

            // this is the result I want to return from a subtree
            // note that this does not depend on acc
            root_val.val + left_sum.max(right_sum)
        }
    }
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut height = 0;
    rec_is_balanced(&root, &mut height)
}

fn rec_is_balanced(root: &Option<Rc<RefCell<TreeNode>>>, height: &mut i32) -> bool {
    match root {
        None => true,
        Some(tree_node) => {
            let root_val = tree_node.as_ref().borrow();
            let mut left_height = 0;
            let mut right_height = 0;
            let mut left_ok = true;
            let mut right_ok = true;

            if let Some(_) = root_val.left {
                left_ok = rec_is_balanced(&root_val.left, &mut left_height);
            }

            if let Some(_) = root_val.right {
                right_ok = rec_is_balanced(&root_val.right, &mut right_height);
            }

            *height = left_height.max(right_height) + 1;

            if (left_height - right_height).abs() > 1 {
                return false;
            }

            left_ok && right_ok
        }
    }
}

pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut to_visit = Vec::from([root]);
    let mut flag = false;

    while !to_visit.is_empty() {
        let node_opt = to_visit.remove(0);

        if let Some(node) = node_opt {
            let node_val = node.as_ref().borrow();

            if let Some(left) = &node_val.left {
                if flag {
                    return false;
                }

                to_visit.push(Some(left.clone()));
            } else { flag = true }

            if let Some(right) = &node_val.right {
                if flag {
                    return false;
                }

                to_visit.push(Some(right.clone()));
            } else { flag = true }
        }
    }

    true
}