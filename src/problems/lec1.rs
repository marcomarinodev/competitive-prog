use std::cmp;

// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/description/
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut max = i32::MIN;
    let mut res = vec![0; arr.len()];

    for i in (0..arr.len()).rev() {
        if i == arr.len() - 1 {
            res[i] = -1;
            max = cmp::max(max, arr[i]);
            continue;
        }

        res[i] = max;
        max = cmp::max(max, arr[i]);
    }

    res
}

/* print all leaders in an array: an element in an array is leader
only if it is greater than all the elements on its right side
*/
pub fn get_leaders(arr: Vec<i32>) -> Vec<i32> {
    let mut max = i32::MIN;
    let mut res = Vec::new();

    for i in (0..arr.len()).rev() {
        if arr[i] > max {
            res.push(arr[i]);
            max = arr[i];
        }
    }

    res
}

/* find the max sum in a sub-array for an input array, f.i:
[-1, 5, 8, -9, 4, 1] ==> 13, [5,8]
*/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    while i < nums.len() && j < nums.len() {
        sum += nums[j];
        max = cmp::max(max, sum);

        j += 1;

        if sum < 0 {
            sum = 0;
            i = j;
        }
    }

    max
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let expected_total = nums.len() * (nums.len() + 1) / 2;
    let mut actual_total = 0;

    for num in nums {
        actual_total += num;
    }

    expected_total as i32 - actual_total
}

// TESTS
#[cfg(test)]
mod lec1_tests {
    use crate::problems::get_leaders;
    use crate::problems::max_sub_array;

    #[test]
    fn get_leaders_test() {
        // Basic test case
        let arr = vec![16, 17, 4, 3, 5, 2];
        assert_eq!(get_leaders(arr), vec![2, 5, 17]);

        // Empty array test case
        let arr = vec![];
        assert_eq!(get_leaders(arr), vec![]);

        // Single element test case
        let arr = vec![5];
        assert_eq!(get_leaders(arr), vec![5]);

        // Negative numbers test case
        let arr = vec![-1, -2, -3, -4, -5];
        assert_eq!(get_leaders(arr), vec![-5, -4, -3, -2, -1]);

        // All equal elements test case
        let arr = vec![3, 3, 3, 3, 3];
        assert_eq!(get_leaders(arr), vec![3]);

        // All decreasing elements test case
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(get_leaders(arr), vec![1, 2, 3, 4, 5]);

        // Mixed positive and negative numbers test case
        let arr = vec![10, -4, 5, -6, 3];
        assert_eq!(get_leaders(arr), vec![3, 5, 10]);
    }

    #[test]
    fn max_sub_array_test() {
        let arr = vec![-1, 5, 8, -9, 4, 1];
        assert_eq!(max_sub_array(arr), 13);
    }
}
