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

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = None;
    let mut cur = &mut result;
    let mut tmp = 0;

    while l1.is_some() || l2.is_some() {
        let val = if l1.is_some() {
            let v = l1.as_ref().unwrap().val;
            l1 = l1.unwrap().next;
            v
        } else {
            0
        } + if l2.is_some() {
            let v = l2.as_ref().unwrap().val;
            l2 = l2.unwrap().next;
            v
        } else {
            0
        } + tmp;
        let new_node = if val > 9 {
            tmp = 1;
            ListNode::new(val - 10)
        } else {
            tmp = 0;
            ListNode::new(val)
        };
        if cur.is_some() {
            cur.as_mut().unwrap().next = Some(Box::new(new_node));
            cur = &mut cur.as_mut().unwrap().next;
        } else {
            *cur = Some(Box::new(new_node));
        }
    }

    if tmp > 0 {
        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(tmp)));
    }

    result
}
