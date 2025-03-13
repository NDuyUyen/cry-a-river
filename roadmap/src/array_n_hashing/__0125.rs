pub fn is_palindrome(s: String) -> bool {
    let mut left_cur = 0;
    let mut right_cur = s.len() - 1;
    let vec_s: Vec<char> = s.to_lowercase().chars().collect();

    while left_cur < right_cur {
        if !vec_s[left_cur].is_alphanumeric() {
            left_cur += 1;
            continue;
        }
        if !vec_s[right_cur].is_alphanumeric() {
            right_cur -= 1;
            continue;
        }

        if vec_s[left_cur] != vec_s[right_cur] {
            return false;
        } else {
            left_cur += 1;
            right_cur -= 1;
        }
    }
    true
}
