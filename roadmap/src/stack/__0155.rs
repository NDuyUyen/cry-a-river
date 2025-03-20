struct MinStack {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        if self.stack.is_empty() {
            -1
        } else {
            self.stack[self.stack.len() - 1]
        }
    }

    fn get_min(&self) -> i32 {
        if self.stack.is_empty() {
            return -1;
        } else {
            let mut result = self.stack[0];
            for i in 1..self.stack.len() {
                if self.stack[i] < result {
                    result = self.stack[i];
                }
            }
            result
        }
    }
}
