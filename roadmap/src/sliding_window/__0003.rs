use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut result = 1;
    let mut cur_pos: HashMap<char, usize> = HashMap::new();
    let mut left_bound = 0;
    s.chars().into_iter().enumerate().for_each(|(i, c)| {
        cur_pos
            .entry(c)
            .and_modify(|cur_i| {
                if *cur_i >= left_bound {
                    let len = (i - left_bound) as i32;
                    if result < len {
                        result = len;
                    }
                    left_bound = *cur_i + 1;
                } else {
                    let len = (i - left_bound + 1) as i32;
                    if result < len {
                        result = len;
                    }
                }
                *cur_i = i;
            })
            .or_insert(i);
    });

    let len = (s.len() - left_bound) as i32;
    if result < len {
        result = len;
    }
    result
}
