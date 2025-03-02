use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut positions: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        positions.insert(nums[i], i as i32);
    }

    for i in 0..nums.len() {
        match positions.get(&(target - nums[i])) {
            Some(p) => {
                if *p != i as i32 {
                    return vec![i as i32, *p];
                }
            }
            None => {}
        }
    }
    vec![]
}
