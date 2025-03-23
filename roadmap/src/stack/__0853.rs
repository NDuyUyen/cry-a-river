pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut pairs = position
        .iter()
        .zip(speed.iter())
        .collect::<Vec<(&i32, &i32)>>();

    pairs.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));

    let mut cur_t = 0.0;
    let mut res = 0;

    pairs.iter().for_each(|(p, s)| {
        let new_t = (target - **p) as f32 / **s as f32;
        if new_t > cur_t {
            res += 1;
            cur_t = new_t;
        }
    });

    res
}
