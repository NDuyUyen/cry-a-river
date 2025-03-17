pub fn trap(height: Vec<i32>) -> i32 {
    let mut left_cur = 0;
    let mut right_cur = height.len() - 1;

    if left_cur < right_cur {
        let mut max_height = if height[left_cur] > height[right_cur] {
            height[right_cur]
        } else {
            height[left_cur]
        };
        let mut result = max_height * (right_cur - left_cur - 1) as i32;

        while left_cur < right_cur {
            if height[left_cur] < height[right_cur] {
                left_cur += 1;
                if left_cur != right_cur {
                    if height[left_cur] <= max_height {
                        result -= height[left_cur];
                    } else {
                        let new_max_height = if height[left_cur] > height[right_cur] {
                            height[right_cur]
                        } else {
                            height[left_cur]
                        };
                        result += (new_max_height - max_height) * (right_cur - left_cur - 1) as i32
                            - max_height;
                        max_height = new_max_height;
                    }
                } else {
                    break;
                }
            } else {
                right_cur -= 1;
                if left_cur != right_cur {
                    if height[right_cur] <= max_height {
                        result -= height[right_cur];
                    } else {
                        let new_max_height = if height[left_cur] > height[right_cur] {
                            height[right_cur]
                        } else {
                            height[left_cur]
                        };
                        result += (new_max_height - max_height) * (right_cur - left_cur - 1) as i32
                            - max_height;
                        max_height = new_max_height;
                    }
                } else {
                    break;
                }
            }
        }
        result
    } else {
        0
    }
}
