use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}
