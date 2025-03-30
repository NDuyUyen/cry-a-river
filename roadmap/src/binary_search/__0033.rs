pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let on_left_side = nums[left] <= target;
    while left < right {
        if nums[left] <= nums[right] {
            break;
        }

        let center = left + (right - left) / 2;
        if nums[center] == target {
            return center as i32;
        } else if nums[center] > nums[right] {
            if on_left_side && target <= nums[center] {
                right = center;
            } else if left == center {
                left = right;
            } else {
                left = center;
            }
        } else {
            if on_left_side || target <= nums[center] {
                right = center;
            } else if left == center {
                left = right;
            } else {
                left = center;
            }
        }
    }

    while left < right {
        let center = left + (right - left) / 2;
        if target > nums[center] {
            if left == center {
                left = right;
            } else {
                left = center;
            }
        } else {
            right = center;
        }
    }

    return if nums[left] == target {
        left as i32
    } else {
        -1
    };
}
