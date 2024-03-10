use std::{cell::RefCell, rc::Rc};

use leetcode::data_structures::TreeNode;

fn main() {
    let tree1 = TreeNode::from_vec(vec![
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ]);

    println!("node: {:#?}", tree1);

    let result = Solution::is_symmetric(tree1);

    println!("result: {:#?}", result);
}

struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => false,
            Some(node) => Self::is_mirror(node.borrow().left.clone(), node.borrow().right.clone()),
        }
    }

    pub fn is_mirror(
        node1: Option<Rc<RefCell<TreeNode>>>,
        node2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (node1, node2) {
            (None, None) => true,
            (Some(node1), Some(node2)) => {
                let left_node = node1.borrow();
                let right_node = node2.borrow();

                if left_node.val != right_node.val {
                    return false;
                }

                 Self::is_mirror(left_node.left.clone(), right_node.right.clone())
                    && Self::is_mirror(left_node.right.clone(), right_node.left.clone())
            }
            _ => false,
        }
    }
}
