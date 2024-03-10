use std::{cell::RefCell, rc::Rc};

use leetcode::data_structures::TreeNode;

fn main() {
    let node = TreeNode::from_vec(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);

    println!("node: {:#?}", node);

    let result = Solution::invert_tree(node);

    println!("result: {:#?}", result);
}

struct Solution;

impl Solution {
    fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // root.left = invertTree(root.right);
        // root.right = invertTree(root.left);

        match root {
            None => (),
            Some(ref node) => {
                let mut node = node.borrow_mut();
                let left = node.left.clone();
                let right = node.right.clone();
                node.left = Self::invert_tree(right);
                node.right = Self::invert_tree(left);
            }
        }

        root
    }
}
