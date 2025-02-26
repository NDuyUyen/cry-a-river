use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        false
    } else {
        let mut map: HashMap<char, (usize, usize)> = HashMap::new();

        s.chars()
            .into_iter()
            .zip(t.chars().into_iter())
            .for_each(|(sc, tc)| {
                map.entry(sc)
                    .and_modify(|i: &mut (usize, usize)| i.0 += 1)
                    .or_insert((1, 0));
                map.entry(tc)
                    .and_modify(|i: &mut (usize, usize)| i.1 += 1)
                    .or_insert((0, 1));
            });

        map.values().all(|(sf, tf)| sf == tf)
    }
}
