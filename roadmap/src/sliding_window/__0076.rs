use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    let mut t_freq: HashMap<char, i32> = HashMap::new();
    t.chars().for_each(|c| {
        t_freq.entry(c).and_modify(|f| *f += 1).or_insert(1);
    });

    let mut left = 0;
    let mut t_len = t.len();
    let mut final_left = 0;
    let mut final_right = s.len() + 1;
    let s: Vec<char> = s.chars().collect();

    while left < s.len() {
        match t_freq.get(&s[left]) {
            Some(_) => break,
            None => left += 1,
        }
    }

    for right in left..s.len() {
        t_freq.entry(s[right]).and_modify(|f| {
            if *f > 0 {
                t_len -= 1;
            }
            *f -= 1;
        });

        while t_len == 0 {
            if right - left < final_right - final_left {
                final_left = left;
                final_right = right;
            }
            t_freq.entry(s[left]).and_modify(|f| {
                if *f == 0 {
                    t_len += 1;
                }
                *f += 1;
            });
            left += 1;
        }
    }

    if final_right == s.len() + 1 {
        String::new()
    } else {
        s.into_iter()
            .skip(final_left)
            .take(final_right - final_left + 1)
            .collect()
    }
}
