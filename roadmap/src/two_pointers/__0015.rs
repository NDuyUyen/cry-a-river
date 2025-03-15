pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut nums = nums;
    nums.sort();

    for i in 0..(nums.len() - 2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left_cur = i + 1;
        let mut right_cur = nums.len() - 1;

        while left_cur < right_cur {
            let sum = nums[i] + nums[left_cur] + nums[right_cur];
            if sum < 0 {
                left_cur += 1;
            } else if sum > 0 {
                right_cur -= 1;
            } else {
                result.push(vec![nums[i], nums[left_cur], nums[right_cur]]);
                while left_cur < right_cur && nums[left_cur] == nums[left_cur + 1] {
                    left_cur += 1;
                }

                while left_cur < right_cur && nums[right_cur] == nums[right_cur - 1] {
                    right_cur -= 1;
                }

                left_cur += 1;
                right_cur -= 1;
            }
        }
    }

    result
}
