
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

            let left_sum = max_sum_helper(&root_val.left, acc).max(0);
            let right_sum = max_sum_helper(&root_val.right, acc).max(0);

            let mut total_sum = root_val.val + left_sum + right_sum;

            // shall I consider the total sum in the path sum
            *acc = *acc.max(&mut total_sum); 

            root_val.val + left_sum.max(right_sum)
        }
    }
}

