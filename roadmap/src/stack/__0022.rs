pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(open: i32, close: i32, stack: &mut Vec<String>, res: &mut Vec<String>) {
        if open == close && open == 0 {
            let valid_p = stack.join("");
            res.push(valid_p);

            return;
        }
        if open > 0 {
            stack.push("(".to_string());
            backtrack(open - 1, close, stack, res);
            stack.pop();
        }
        if open < close {
            stack.push(")".to_string());
            backtrack(open, close - 1, stack, res);
            stack.pop();
        }
    }
    let mut result: Vec<String> = vec![];
    backtrack(n, n, &mut vec![], &mut result);
    result
}
