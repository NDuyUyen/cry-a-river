pub fn reverse_words(s: String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut end = 0;
    let mut start = 0;

    while end < s.len() {
        if chars[end] == ' ' {
            if start == end {
                start += 1;
                end += 1;
            } else {
                let sub_s: String = chars[start..end].iter().collect();
                if result.len() > 0 {
                    result = sub_s + " " + &result;
                } else {
                    result = sub_s;
                }
                end += 1;
                start = end;
            }
        } else {
            end += 1;
        }
    }

    if start < s.len() {
        let sub_s: String = chars[start..s.len()].iter().collect();
        if result.len() > 0 {
            result = sub_s + " " + &result;
        } else {
            result = sub_s;
        }
    }
    result
}
