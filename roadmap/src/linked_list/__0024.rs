#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = &mut head;

    while cur.is_some() {
        if cur.as_mut().unwrap().next.is_some() {
            let cur_val = cur.as_mut().unwrap().val;
            cur.as_mut().unwrap().val = cur.as_mut().unwrap().next.as_mut().unwrap().val;
            cur.as_mut().unwrap().next.as_mut().unwrap().val = cur_val;
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        } else {
            break;
        }
    }
    head
}
