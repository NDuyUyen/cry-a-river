pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let center = left + (right - left) / 2;
        if nums[right] < nums[center] {
            if left == center {
                left = right;
            } else {
                left = center;
            }
        } else {
            right = center;
        }
    }

    nums[left]
}
