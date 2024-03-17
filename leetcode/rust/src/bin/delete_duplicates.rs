use leetcode::data_structures::list::ListNode;

fn main() {
    let nodes = ListNode::from_array(vec![1, 1, 1, 4]);

    let result = Solution::delete_duplicates(nodes);

    println!("{:#?}", result);
}

struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;

        while let Some(ref mut cur_node) = current {
            while let Some(ref next_node) = cur_node.next {
                if cur_node.val == next_node.val {
                    cur_node.next = next_node.next.clone();
                } else {
                    break;
                }
            }
            current = &mut cur_node.next;
        }

        head
    }
}
