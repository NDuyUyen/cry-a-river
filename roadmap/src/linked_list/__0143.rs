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

pub fn split_and_reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut step_2 = &head.clone();
    let mut step_1 = head;

    while step_2.is_some() {
        step_2 = &step_2.as_ref().unwrap().next;
        if step_2.is_some() {
            step_2 = &step_2.as_ref().unwrap().next;
            step_1 = &mut (step_1.as_mut().unwrap().next);
        }
    }
    let mut head2: Option<Box<ListNode>> = step_1.as_mut().unwrap().next.take();
    let mut reversed_head2 = None;

    while let Some(mut node) = head2 {
        head2 = node.next;
        node.next = reversed_head2;
        reversed_head2 = Some(node);
    }

    reversed_head2
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut cur2 = split_and_reverse(head);
    let mut cur1 = head;
    while cur1.is_some() && cur2.is_some() {
        let next1 = cur1.as_mut().unwrap().next.take();
        let next2 = cur2.as_mut().unwrap().next.take();

        cur1.as_mut().unwrap().next = cur2;
        cur1.as_mut().unwrap().next.as_mut().unwrap().next = next1;
        cur1 = &mut cur1.as_mut().unwrap().next.as_mut().unwrap().next;
        cur2 = next2;
    }
}
