pub fn score_of_parentheses(s: String) -> i32 {
    let mut stack: Vec<i32> = vec![];
    let mut cur = 0;
    s.chars().into_iter().for_each(|c| {
        if c == '(' {
            stack.push(cur);
            cur = 0;
        } else {
            cur += match stack.pop() {
                Some(top) => top,
                None => 0,
            } + if cur > 1 { cur } else { 1 };
        }
    });

    cur
}
