
// https://www.geeksforgeeks.org/problems/maximum-path-sum/1
use std::rc::Rc;
use std::cell::RefCell;

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

