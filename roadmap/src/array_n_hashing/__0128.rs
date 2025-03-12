use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut result = 0;

    set.iter().for_each(|n| {
        if !set.contains(&(n - 1)) {
            let mut length = 1;
            while set.contains(&(n + length)) {
                length += 1;
            }

            if length > result {
                result = length;
            }
        }
    });

    result
}
