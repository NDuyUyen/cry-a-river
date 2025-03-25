pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut stack: Vec<usize> = vec![];

    for i in 0..heights.len() {
        loop {
            match stack.last() {
                Some(j) => {
                    if heights[*j] > heights[i] {
                        let height = heights[*j];
                        stack.pop();
                        let pse = match stack.last() {
                            Some(z) => *z as i32,
                            None => -1,
                        };

                        let new_area = (i as i32 - pse - 1) * height;
                        if new_area > result {
                            result = new_area;
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        stack.push(i);
    }

    loop {
        match stack.last() {
            Some(j) => {
                let height = heights[*j];
                stack.pop();
                let pse = match stack.last() {
                    Some(z) => *z as i32,
                    None => -1,
                };

                let new_area = (heights.len() as i32 - pse - 1) * height;
                if new_area > result {
                    result = new_area;
                }
            }
            None => break,
        }
    }
    result
}
