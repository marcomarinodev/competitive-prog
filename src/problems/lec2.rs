

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

