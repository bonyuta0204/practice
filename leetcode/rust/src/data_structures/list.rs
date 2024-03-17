// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_array(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        for val in vals {
            let new_node = Box::new(ListNode::new(val));
            *tail = Some(new_node);
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}
