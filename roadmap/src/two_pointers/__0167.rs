pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left_cur = 0;
    let mut right_cur = numbers.len() - 1;

    while left_cur < right_cur {
        if numbers[left_cur] + numbers[right_cur] < target {
            left_cur += 1;
        } else if numbers[left_cur] + numbers[right_cur] > target {
            right_cur -= 1;
        } else {
            return vec![left_cur as i32 + 1, right_cur as i32 + 1];
        }
    }
    vec![]
}
