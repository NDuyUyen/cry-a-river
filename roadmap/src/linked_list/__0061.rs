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

pub fn length(head: &mut Option<Box<ListNode>>) -> i32 {
    let mut length = 0;
    let mut cur = head;

    while let Some(node) = cur {
        length += 1;
        cur = &mut node.next;
    }
    length
}

pub fn split(head: &mut Option<Box<ListNode>>, p: i32) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut cnt = 0;
    while let Some(node) = cur {
        cnt += 1;
        if cnt == p {
            return if node.next.is_some() {
                node.next.take()
            } else {
                None
            };
        }
        cur = &mut node.next;
    }
    cur.take()
}

pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let length = length(&mut head);

    if length > 0 {
        let split_at = (length - (k % length)) % length;
        if split_at == 0 {
            head
        } else {
            let mut tail = split(&mut head, split_at);
            let mut cur = &mut tail;
            if cur.is_some() {
                loop {
                    if cur.as_ref().unwrap().next.is_some() {
                        cur = &mut cur.as_mut().unwrap().next;
                    } else {
                        cur.as_mut().unwrap().next = head.take();
                        break;
                    }
                }
            }
            tail
        }
    } else {
        None
    }
}
