use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut s1_freq: HashMap<char, i32> = HashMap::new();
    s1.chars().for_each(|c| {
        s1_freq.entry(c).and_modify(|f| *f += 1).or_insert(1);
    });

    let mut s1_len = s1.len();
    let s2: Vec<char> = s2.chars().collect();
    let mut left = 0;

    for right in 0..s2.len() {
        let mut out_of_stock = false;
        let mut unexpected_char = true;
        s1_freq.entry(s2[right]).and_modify(|f| {
            if *f > 0 {
                *f -= 1;
                s1_len -= 1;
            } else {
                out_of_stock = true;
            }
            unexpected_char = false;
        });

        if s1_len == 0 {
            return true;
        }

        if unexpected_char {
            while left <= right {
                s1_freq.entry(s2[left]).and_modify(|f| *f += 1);
                left += 1;
            }
            s1_len = s1.len();
        } else if out_of_stock {
            while left <= right {
                if s2[left] == s2[right] {
                    left += 1;
                    break;
                } else {
                    s1_freq.entry(s2[left]).and_modify(|f| *f += 1);
                }
                left += 1;
                s1_len += 1;
            }
        }
    }
    false
}
