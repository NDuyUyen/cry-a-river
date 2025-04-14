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

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_some() {
        let mut cur = &mut head;

        while cur.is_some() {
            let cur_val = cur.as_mut().unwrap().val;
            let mut next_cur = &mut cur.as_mut().unwrap().next;

            while next_cur.is_some() {
                if next_cur.as_mut().unwrap().val == cur_val {
                    next_cur = &mut next_cur.as_mut().unwrap().next;
                } else {
                    break;
                }
            }
            cur.as_mut().unwrap().next = next_cur.take();
            cur = &mut cur.as_mut().unwrap().next;
        }
    }
    head
}
