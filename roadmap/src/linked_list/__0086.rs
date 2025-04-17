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

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut left: Option<Box<ListNode>> = None;
    let mut left_cur = &mut left;
    let mut right: Option<Box<ListNode>> = None;
    let mut right_cur = &mut right;
    let cur = &mut head;
    while cur.is_some() {
        if cur.as_mut().unwrap().val < x {
            if left_cur.is_some() {
                left_cur.as_mut().unwrap().next = cur.take();
                left_cur = &mut left_cur.as_mut().unwrap().next;
            } else {
                *left_cur = cur.take();
            }
            *cur = left_cur.as_mut().unwrap().next.take();
        } else {
            if right_cur.is_some() {
                right_cur.as_mut().unwrap().next = cur.take();
                right_cur = &mut right_cur.as_mut().unwrap().next;
            } else {
                *right_cur = cur.take();
            }
            *cur = right_cur.as_mut().unwrap().next.take();
        }
    }
    if left_cur.is_some() {
        left_cur.as_mut().unwrap().next = right.take();
        left
    } else {
        right
    }
}
