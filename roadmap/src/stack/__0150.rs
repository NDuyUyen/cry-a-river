pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    tokens.iter().for_each(|t| {
        if *t == "+" {
            match (stack.pop(), stack.pop()) {
                (Some(o1), Some(o2)) => {
                    stack.push(o1 + o2);
                }
                _ => {}
            }
        } else if *t == "-" {
            match (stack.pop(), stack.pop()) {
                (Some(o1), Some(o2)) => {
                    stack.push(o2 - o1);
                }
                _ => {}
            }
        } else if *t == "*" {
            match (stack.pop(), stack.pop()) {
                (Some(o1), Some(o2)) => {
                    stack.push(o1 * o2);
                }
                _ => {}
            }
        } else if *t == "/" {
            match (stack.pop(), stack.pop()) {
                (Some(o1), Some(o2)) => {
                    stack.push(o2 / o1);
                }
                _ => {}
            }
        } else {
            let o: i32 = t.parse().unwrap();
            stack.push(o);
        }
    });

    stack.pop().unwrap()
}
