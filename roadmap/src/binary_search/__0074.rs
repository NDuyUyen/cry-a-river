pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for i in 0..matrix.len() {
        let mut left = 0;
        let mut right = matrix[i].len() - 1;

        if matrix[i][left] <= target && matrix[i][right] >= target {
            while left < right {
                let center = left + (right - left) / 2;
                if matrix[i][center] == target {
                    return true;
                } else if matrix[i][center] < target {
                    if left < center {
                        left = center;
                    } else {
                        left = right;
                    }
                } else {
                    right = center;
                }
            }
            return matrix[i][left] == target;
        } else if matrix[i][left] > target {
            break;
        }
    }
    return false;
}
