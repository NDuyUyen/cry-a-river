pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let sum: i64 = piles.iter().map(|p| *p as i64).sum();
    let mut left = (sum / h as i64 + if sum % h as i64 == 0 { 0 } else { 1 }) as i32;
    // len * (max / h) >= k
    // len <= h
    // => k <= max
    let mut right = *piles.iter().max().unwrap();

    while left < right {
        let k = left + (right - left) / 2;
        let sum_h: i32 = piles
            .iter()
            .map(|p| p / k + if p % k == 0 { 0 } else { 1 })
            .sum();

        if sum_h == h {
            right = k;
        } else if sum_h > h {
            if left < k {
                left = k;
            } else {
                left = right;
            }
        } else {
            if right > k {
                right = k;
            } else {
                right = left;
            }
        }
    }
    return left;
}
