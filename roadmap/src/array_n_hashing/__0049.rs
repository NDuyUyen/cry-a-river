use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    strs.into_iter().for_each(|s| {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        map.entry(key)
            .and_modify(|v| v.push(s.to_string()))
            .or_insert(vec![s.to_string()]);
    });

    map.into_iter().map(|(_, v)| v).collect()
}
