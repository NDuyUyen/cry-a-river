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

pub fn reverse_and_merge(
    head: Option<Box<ListNode>>,
    tail: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut new_head: Option<Box<ListNode>> = tail;
    let mut cur = head;

    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = new_head;
        new_head = Some(node);
    }
    new_head
}

pub fn reverse_between(
    mut head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    if left == right {
        head.take()
    } else {
        let mut head_cur = &mut head;
        let mut i = 1;
        let mut mid: Option<Box<ListNode>> = None;
        let mid_cur = &mut mid;
        let mut tail: Option<Box<ListNode>> = None;

        if left == 1 {
            mid = head_cur.take();
        } else {
            loop {
                i += 1;
                if head_cur.is_none() {
                    break;
                } else if i == left {
                    mid = head_cur.as_mut().unwrap().next.take();
                    break;
                } else {
                    head_cur = &mut head_cur.as_mut().unwrap().next;
                }
            }
        }
        let mut mid_cur = &mut mid;

        loop {
            if mid_cur.is_none() {
                break;
            } else if i == right {
                tail = mid_cur.as_mut().unwrap().next.take();
                break;
            } else {
                i += 1;
                mid_cur = &mut mid_cur.as_mut().unwrap().next;
            }
        }

        if head_cur.is_some() {
            head_cur.as_mut().unwrap().next = reverse_and_merge(mid, tail);
            head
        } else {
            reverse_and_merge(mid, tail)
        }
    }
}
