pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left_cur = 0;
    let mut right_cur = height.len() - 1;
    let mut result = 0;

    while left_cur < right_cur {
        let new_vol = ((right_cur - left_cur) as i32)
            * if height[left_cur] < height[right_cur] {
                let min_height = height[left_cur];

                while left_cur < right_cur && height[left_cur] <= min_height {
                    left_cur += 1;
                }
                min_height
            } else {
                let min_height = height[right_cur];
                while left_cur < right_cur && height[right_cur] <= min_height {
                    right_cur -= 1;
                }
                min_height
            };

        if new_vol > result {
            result = new_vol;
        }
    }
    result
}
