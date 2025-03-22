pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut result = vec![0; temperatures.len()];

    for i in 0..temperatures.len() {
        while !stack.is_empty() {
            match stack.pop() {
                Some(cur_i) => {
                    if temperatures[i] > temperatures[cur_i] {
                        result[cur_i] = (i - cur_i) as i32;
                    } else {
                        stack.push(cur_i);
                        break;
                    }
                }
                None => break,
            }
        }
        stack.push(i);
    }

    result
}
