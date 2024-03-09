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

    let result = Solution::max_depth(node);

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
