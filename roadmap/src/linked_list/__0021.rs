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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(node1), Some(node2)) => {
            if node1.val < node2.val {
                Some(Box::new(ListNode {
                    val: node1.val,
                    next: merge_two_lists(node1.next, Some(node2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: node2.val,
                    next: merge_two_lists(Some(node1), node2.next),
                }))
            }
        }
        (Some(node), None) | (None, Some(node)) => Some(node),
        (_, _) => None,
    }
}
