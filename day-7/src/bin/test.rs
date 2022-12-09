// Definition for singly-linked list.
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
pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut even_head = ListNode::new(-1);
    let mut odd_head = ListNode::new(-1);
    let mut even_curr = &mut even_head;
    let mut odd_curr = &mut odd_head;
    let mut is_even = false;
    let mut t = &head;

    while let Some(node) = t {
        t = &node.next;
        if is_even {
            even_curr.next = Some(node.to_owned());
            even_curr = even_curr.next.as_mut().unwrap();
        } else {
            odd_curr.next = Some(node.to_owned());
            odd_curr = odd_curr.next.as_mut().unwrap();
        }
        is_even = !is_even;
    }

    even_curr.next = None;
    odd_curr.next = even_head.next;
    return odd_head.next;
}
pub fn main() {}
