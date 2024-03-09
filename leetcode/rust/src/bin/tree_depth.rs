use std::{cell::RefCell, rc::Rc};

use leetcode::data_structures::TreeNode;

fn main() {
    let node1 = TreeNode::new(3);
    let node2 = TreeNode::new(5);
    let node3 = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(node1))),
        right: Some(Rc::new(RefCell::new(node2))),
    };

    let result = Solution::max_depth(Some(Rc::new(RefCell::new(node3))));

    println!("result: {}", result);
}

struct Solution;

impl Solution {
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = node.left.as_ref();
                let right = node.right.as_ref();

                if left.is_none() && right.is_none() {
                    1
                } else {
                    let left_max = left
                        .map(|left| Self::max_depth(Some(left.clone())))
                        .unwrap_or(0);
                    let right_max = right
                        .map(|right| Self::max_depth(Some(right.clone())))
                        .unwrap_or(0);

                    if left_max > right_max {
                        left_max + 1
                    } else {
                        right_max + 1
                    }
                }
            }
        }
    }
}
