use crate::data_structures::list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_helper(l1, l2, 0)
    }

    fn add_two_numbers_helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if let Some(node1) = l1 {
            if let Some(node2) = l2 {
                // both nodes exist
                let sum = node1.val + node2.val + carry;
                let carry = sum / 10;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add_two_numbers_helper(node1.next, node2.next, carry),
                }))
            } else {
                // only node1 exists
                let sum = node1.val + carry;
                let carry = sum / 10;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add_two_numbers_helper(node1.next, None, carry),
                }))
            }
        } else if let Some(node2) = l2 {
            let sum = node2.val + carry;
            let carry = sum / 10;
            Some(Box::new(ListNode {
                val: sum % 10,
                next: Solution::add_two_numbers_helper(None, node2.next, carry),
            }))
        } else {
            if carry == 0 {
                None
            } else {
                Some(Box::new(ListNode::new(carry)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        let node1 = ListNode::new(2);
        let node2 = ListNode::new(5);
        let node3 = ListNode::new(8);

        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(node1.clone())), Some(Box::new(node2))),
            Some(Box::new(ListNode::new(7)))
        );

        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(node1.clone())), Some(Box::new(node3))),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1)))
            }))
        );
    }

    #[test]
    fn test_diffrent_length() {
        let node1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(1))),
        };

        let node2 = ListNode::new(8);

        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(node1.clone())), Some(Box::new(node2.clone()))),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(2)))
            }))
        );

        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(node2.clone())), Some(Box::new(node1.clone()))),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(2)))
            }))
        )
    }
}
