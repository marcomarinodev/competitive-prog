use std::collections::{HashMap, VecDeque};

pub fn trapping_rain_water(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_left = height[left];
    let mut max_right = height[right];

    while left < right {
        if height[left] > height[right] {
            right -= 1;
            if height[right] < max_right {
                res += max_right - height[right];
            } else {
                max_right = height[right];
            }
        } else {
            left += 1;
            // since the left bound is lower then the right one
            // so I calculate the difference between the left bound and
            // the current index
            if height[left] < max_left {
                res += max_left - height[left];
            } else {    
                // otherwise update left bound
                max_left = height[left];
            }
        }

    }

    res
}

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res =  Vec::new();
    let mut deque: VecDeque<(i32, usize)> = VecDeque::new();
    let mut start_window_index = 0;

    for (i, num) in nums.iter().enumerate() {

        // remove element not in window
        while let Some(&(_element, position)) = deque.front() {
            if position >= start_window_index {
                break;
            }
            deque.pop_front();
        }

        // insert the new element in the window, bubbling it up from the tail
        // and removing all the ones that has value smaller than the element we are 
        // inserting
        while let Some(&(last_element, _)) = deque.back() {
            if last_element >= *num {
                break;
            }
            deque.pop_back();
        }
        deque.push_back((*num, i));

        if i >= (k - 1) as usize {
            // insert max in res
            if let Some(max_pair) = deque.front() {
                res.push(max_pair.0);
            }

            // move forward start_window_index
            start_window_index += 1;
        }
    }

    res
}

// https://leetcode.com/problems/next-greater-element-i/
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (mut num_map, mut stack) = (HashMap::new(), vec![]);
    let mut res = vec![-1; nums1.len()];

    nums1.iter().enumerate().for_each(|(i, &num)| {
        num_map.insert(num, i);
    });

    nums2.into_iter().for_each(|num| {
        while let Some(&last) = stack.last() {
            if num > last { // num is the next greater element of the last
                let popped_last = stack.pop().unwrap();
                if let Some(last_idx) = num_map.get(&popped_last) {
                    res[*last_idx] = num;
                }
            } else {
                break;
            }
        }
        stack.push(num);
    });

    res 
}

