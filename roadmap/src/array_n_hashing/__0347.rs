use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|n| {
        freq_map.entry(*n).and_modify(|f| *f -= 1).or_insert(-1);
    });
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    freq_map.iter().for_each(|(n, f)| {
        heap.push((*f, *n));

        if heap.len() as i32 > k {
            heap.pop();
        }
    });

    heap.iter().map(|(_, n)| *n).collect()
}
