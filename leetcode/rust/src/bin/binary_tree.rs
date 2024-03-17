use std::{cell::RefCell, rc::Rc, slice::Iter};

use leetcode::data_structures::TreeNode;

fn main() {
    let preorder = vec![3, 9, 20, 15, 7];

    let inorder = vec![9, 3, 15, 20, 7];

    let result = Solution::build_tree(preorder, inorder);

    println!("result: {:#?}", result);
}

struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder.iter();
        Self::build_tree_internal(&mut preorder, inorder)
    }

    fn build_tree_internal(
        preorder: &mut Iter<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let current_node = preorder.next();

        match current_node {
            Some(node) => {
                let mut tree = TreeNode::new(*node);
                // Split inorder into left and right. split inorder by current_node

                let pos = inorder.iter().position(|&x| x == *node).unwrap();
                let left_inorder = &inorder[..pos];
                let right_inorder = &inorder[pos + 1..];

                if left_inorder.len() > 0 {
                    tree.left = Self::build_tree_internal(preorder, left_inorder.to_vec());
                }
                if right_inorder.len() > 0 {
                    tree.right = Self::build_tree_internal(preorder, right_inorder.to_vec());
                }

                Some(Rc::new(RefCell::new(tree)))
            }
            None => {
                return None;
            }
        }
    }
}
