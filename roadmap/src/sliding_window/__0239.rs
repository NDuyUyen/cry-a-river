use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let k_us = k as usize;
    let mut q: VecDeque<i32> = VecDeque::new();

    for i in 0..nums.len() {
        if i >= k as usize && nums[i - k_us] == q[0] {
            q.pop_front();
        }

        while let Some(&n) = q.back() {
            if n < nums[i] {
                q.pop_back();
            } else {
                break;
            }
        }

        q.push_back(nums[i]);

        if i >= k_us - 1 {
            result.push(q[0]);
        }
    }
    result
}
