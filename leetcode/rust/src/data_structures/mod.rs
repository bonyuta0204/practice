use std::{cell::RefCell, rc::Rc};

pub mod list;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 0, 1, [2, 3], [4, 5], [[6, 7]], [8, 9]]
impl TreeNode {
    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = v
            .into_iter()
            .map(|x| x.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))))
            .collect();
        let root = nodes.remove(0).unwrap();
        for i in 0..nodes.len() {
            let mut node = nodes[i].as_ref();
            if let Some(ref mut n) = node {
                match i {
                    0 => root.borrow_mut().left = Some(Rc::clone(n)),
                    1 => root.borrow_mut().right = Some(Rc::clone(n)),
                    i => {
                        if i % 2 == 0 {
                            nodes[(i / 2) - 1].as_ref().unwrap().borrow_mut().left =
                                Some(Rc::clone(n));
                        } else {
                            nodes[(i / 2) - 1].as_ref().unwrap().borrow_mut().right =
                                Some(Rc::clone(n));
                        }
                    }
                }
            }
        }
        Some(root)
    }
}
