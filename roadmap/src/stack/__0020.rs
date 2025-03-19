pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let s: Vec<char> = s.chars().into_iter().collect();

    for i in 0..s.len() {
        if s[i] == ')' {
            match stack.pop() {
                Some('(') => {}
                _ => {
                    return false;
                }
            }
        } else if s[i] == ']' {
            match stack.pop() {
                Some('[') => {}
                _ => {
                    return false;
                }
            }
        } else if s[i] == '}' {
            match stack.pop() {
                Some('{') => {}
                _ => {
                    return false;
                }
            }
        } else {
            stack.push(s[i]);
        }
    }

    return stack.len() == 0;
}
