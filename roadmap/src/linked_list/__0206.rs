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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head: Option<Box<ListNode>> = None;
    let mut cursor = head;

    while let Some(mut node) = cursor {
        cursor = node.next.take();
        node.next = new_head;
        new_head = Some(node);
    }
    new_head
}
