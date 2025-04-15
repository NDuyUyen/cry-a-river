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

pub fn next_difference(head: &mut Option<Box<ListNode>>) {
    while head.is_some() {
        let cur_val = head.as_mut().unwrap().val;
        let mut next = &mut head.as_mut().unwrap().next;
        let mut cnt = 1;
        while let Some(next_node) = next {
            if next_node.val == cur_val {
                next = &mut next.as_mut().unwrap().next;
                cnt += 1;
            } else {
                break;
            }
        }
        if cnt == 1 {
            break;
        } else {
            *head = next.take();
        }
    }
}

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    next_difference(&mut head);
    let mut cur = &mut head;

    while cur.is_some() {
        let mut next = &mut cur.as_mut().unwrap().next;
        next_difference(&mut next);
        cur.as_mut().unwrap().next = next.take();
        cur = &mut cur.as_mut().unwrap().next;
    }
    head
}
