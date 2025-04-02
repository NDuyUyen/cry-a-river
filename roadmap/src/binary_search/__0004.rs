pub fn find_max_left(nums1: &Vec<i32>, nums2: &Vec<i32>, idx_1: usize, idx_2: usize) -> i32 {
    let max_left_1 = if idx_1 > 0 {
        nums1[idx_1 - 1]
    } else {
        i32::MIN
    };

    let max_left_2 = if idx_2 > 0 {
        nums2[idx_2 - 1]
    } else {
        i32::MIN
    };

    if max_left_1 > max_left_2 {
        max_left_1
    } else {
        max_left_2
    }
}

pub fn find_min_right(nums1: &Vec<i32>, nums2: &Vec<i32>, idx_1: usize, idx_2: usize) -> i32 {
    let min_right_1 = if idx_1 < nums1.len() {
        nums1[idx_1]
    } else {
        i32::MAX
    };

    let min_right_2 = if idx_2 < nums2.len() {
        nums2[idx_2]
    } else {
        i32::MAX
    };

    if min_right_1 > min_right_2 {
        min_right_2
    } else {
        min_right_1
    }
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let med_idx = (nums1.len() + nums2.len() + 1) / 2;
    let mut idx_2 = 0;
    let mut idx_1 = if med_idx < nums1.len() {
        med_idx
    } else {
        idx_2 = med_idx - nums1.len();
        nums1.len()
    };

    let mut max_left = find_max_left(&nums1, &nums2, idx_1, idx_2);
    let mut min_right = find_min_right(&nums1, &nums2, idx_1, idx_2);

    while max_left > min_right {
        if idx_1 > 0 {
            idx_1 -= 1;
        }
        if idx_2 < nums2.len() {
            idx_2 += 1;
        }
        max_left = find_max_left(&nums1, &nums2, idx_1, idx_2);
        min_right = find_min_right(&nums1, &nums2, idx_1, idx_2);
    }

    if (nums1.len() + nums2.len()) % 2 == 0 {
        (min_right + max_left) as f64 / 2.0
    } else {
        max_left as f64
    }
}
