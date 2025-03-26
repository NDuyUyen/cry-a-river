pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let center = left + (right - left) / 2;
        if nums[center] == target {
            return center as i32;
        } else if nums[center] < target {
            if left < center {
                left = center;
            } else {
                left = right;
            }
        } else {
            right = center;
        }
    }

    if nums[left] == target {
        return left as i32;
    } else {
        return -1;
    }
}
