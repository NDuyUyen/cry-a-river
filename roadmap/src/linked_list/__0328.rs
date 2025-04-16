use std::mem::take;

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

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let cur = &mut head;
    if cur.is_some() {
        let mut even: Option<Box<ListNode>> = None;
        let mut even_cur = &mut even;
        let mut odd: Option<Box<ListNode>> = None;
        let mut odd_cur = &mut odd;

        while cur.is_some() {
            if even_cur.is_none() {
                *even_cur = cur.take();
            } else {
                even_cur.as_mut().unwrap().next = cur.take();
                even_cur = &mut even_cur.as_mut().unwrap().next;
            }
            *cur = even_cur.as_mut().unwrap().next.take();

            if cur.is_some() {
                if odd_cur.is_none() {
                    *odd_cur = cur.take();
                } else {
                    odd_cur.as_mut().unwrap().next = cur.take();
                    odd_cur = &mut odd_cur.as_mut().unwrap().next;
                }
                *cur = odd_cur.as_mut().unwrap().next.take();
            } else {
                break;
            }
        }
        even_cur.as_mut().unwrap().next = odd.take();
        even
    } else {
        cur.take()
    }
}
