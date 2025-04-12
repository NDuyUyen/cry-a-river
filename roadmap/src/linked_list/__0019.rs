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

pub fn _remove_nth_from_end(head: &mut Option<Box<ListNode>>, n: i32) {
    let mut fast = &head.clone();
    let mut slow = head;
    let mut cnt = 0;

    while fast.is_some() {
        fast = &fast.as_ref().unwrap().next;
        cnt += 1;
        if cnt == n {
            break;
        }
    }

    if fast.is_some() {
        loop {
            fast = &fast.as_ref().unwrap().next;
            if fast.is_some() {
                slow = &mut slow.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        if slow.is_some() {
            let next = slow.as_mut().unwrap().next.take();
            let mut next2 = None;
            if next.is_some() {
                next2 = next.unwrap().next;
            }

            slow.as_mut().unwrap().next = next2;
        }
    } else {
        if slow.is_some() {
            *slow = slow.as_mut().unwrap().next.take();
        }
    }
}
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    _remove_nth_from_end(&mut head, n);
    head
}
