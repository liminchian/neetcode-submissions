// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut encoded = vec![];
        let mut q = VecDeque::new();
        q.push_back(root);
        while let Some(node) = q.pop_front() {
            match node {
                None => encoded.push("_".to_string()),
                Some(node) => {
                    let val = node.borrow().val.to_string();
                    encoded.push(val);
                    q.push_back(node.borrow().left.clone());
                    q.push_back(node.borrow().right.clone());
                }
            }
        }
        encoded.join("#")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v = data.split("#");
        let root = match v.next() {
            Some("_") | None => return None,
            Some(val) => {
                Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap())))
            }
        };
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        while let Some(node) = q.pop_front() {
            if let Some(val) = v.next() && val != "_" {
                let left = Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap())));
                node.borrow_mut().left = Some(left.clone());
                q.push_back(left);
            }
            if let Some(val) = v.next() && val != "_" {
                let right = Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap())));
                node.borrow_mut().right = Some(right.clone());
                q.push_back(right);
            }
        }
        Some(root)
    }
}
