pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() > 1 {
        let mut result: Vec<i32> = vec![];
        let mut rev_prd: Vec<i32> = vec![];
        result.push(1);
        for i in 1..nums.len() {
            result.push(result[i - 1] * nums[i - 1]);
        }

        rev_prd.push(1);
        for i in (0..(nums.len() - 1)).rev() {
            rev_prd.push(rev_prd[nums.len() - i - 2] * nums[i + 1]);
            result[i] *= rev_prd[nums.len() - i - 1];
        }

        result
    } else if nums.len() == 1 {
        vec![1]
    } else {
        vec![]
    }
}
