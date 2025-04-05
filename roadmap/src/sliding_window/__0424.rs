use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut result = 0;
    let mut left = 0;
    let mut freq: HashMap<char, i32> = HashMap::new();
    let mut max_freq = 0;
    let s_list: Vec<char> = s.chars().collect();

    for right in 0..s_list.len() {
        freq.entry(s_list[right])
            .and_modify(|f| {
                *f += 1;
                if max_freq < *f {
                    max_freq = *f;
                }
            })
            .or_insert({
                if max_freq < 1 {
                    max_freq = 1;
                }
                1
            });

        let len = (right - left) as i32 + 1;
        if len - max_freq > k {
            freq.entry(s_list[left])
                .and_modify(|f| {
                    *f -= 1;
                })
                .or_insert(0);
            left += 1;
        } else {
            if result < len {
                result = len;
            }
        }
    }

    result
}
