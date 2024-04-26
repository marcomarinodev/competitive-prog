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
    let mut res =  Vec::new(); // vec![0; nums.len() - k + 1]
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

// https://leetcode.com/problems/next-greater-element-ii/description/
// (***) read below
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    let mut stack: Vec<(i32, usize)> = Vec::new();

    for (pos, num) in nums.iter().enumerate() {
        // pop the top until num is greater
        // for each popped element, put in its res position the num value
        // as it represents the next larger
        // worse case: n - 1 elements in decreasing order and then the last one
        // is greater than everyone
        while let Some(&last) = stack.last() {
            // case num is bigger than last.value
            if *num > last.0  {
                // in last.1 position of res, put num
                res[last.1] = *num;
                // pop the element
                stack.pop();
            } else {
                break;
            }
        }

        // insert <num, pos>   
        stack.push((*num, pos));
    }

    // manage circularity
    // TODO: save the index of the inner for loop so that you can avoid useless iterations
    for _ in 0..stack.len() {
        // current
        let current = stack.pop().unwrap();
        let mut found = false;

        for num in &nums {
            if current.0 < *num {
                res[current.1] = *num;
                found = true;
            }
        }
        
        if !found {
            res[current.1] = -1;
        }

    }

    res       
}

// TESTS
#[cfg(test)]
mod lec2_tests {
    use crate::problems::next_greater_elements;

    #[test]
    fn next_greater_elements_test() {
        let arr = vec![1, 2, 1];
        assert_eq!(next_greater_elements(arr), vec![2, -1, 2]);

        let arr = vec![1,2,3,2,1];
        assert_eq!(next_greater_elements(arr), vec![2,3,-1,3,2]);


    }
}

/* (***)
The key is to read the input as a doubled array
[1,2,5,1,4,3, 1,2,5,1,4,3]
[2,5, ,4,5,5]

cur = <1:0>
[<1:0>

cur = 2:1
[2:1

cur 5:2
[5:2

cur 1:3
[5:2, 1:3

cur 4:4

[5:2, 4:4

cur 3:5

[5:2, 4:4, 3:5

3:5 vs 5:8 ==> in position 5 there's gonna be 5

[5:2, 4:4

4:4 vs 5:8 ==> in position 4 there's gonna be 5

[5:2
<v:p> vs <v',p'>
 5:2  vs 5:8 ==> is not greater than ==> go ahead until the array ends

until the end...
============================================
[1,2,3,2,1, 1,2,3,2,1]
[2,3,-1,3,2]

cur = 1:0
[1:0

cur = 2:1
[2:1

cur = 3:2

[3:2

cur = 2:3

[3:2, 2:3

cur 1:4

[3:2, 2:3, 1:4

1:4 vs 1:5 ? is not greater than, go ahead
1:4 vs 2:6 ? yes it is greater so in position 4 there's gonna be 2

[3:2, 2:3

2:3 vs 2:6 ? is not greater, go 
2:3 vs 3:7 ? is greater so in position 3 there's gonna be 3

[3:2

3:2 vs 3:7 ? not greater, go
...
3:2 didn't find any entry greater than him, so in position 2 there's gonna be -1

*/

