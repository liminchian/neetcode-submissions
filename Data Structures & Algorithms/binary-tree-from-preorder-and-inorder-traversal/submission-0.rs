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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mp: HashMap<_, _> = inorder.iter().enumerate().map(|(idx, val)| (*val, idx)).collect();
        let mut pre_idx = 0;

        fn helper(
            preorder: &Vec<i32>,
            pre_idx: &mut usize,
            in_left: i32,
            in_right: i32,
            mp: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if in_left > in_right {
                return None;
            }
            let root_val = preorder[*pre_idx];
            let mut root = TreeNode::new(root_val);
            *pre_idx += 1;
            let root_idx = *mp.get(&root_val).unwrap() as i32;
            root.left = helper(preorder, pre_idx, in_left, root_idx - 1, mp);
            root.right = helper(preorder, pre_idx, root_idx + 1, in_right, mp);
            Some(Rc::new(RefCell::new(root)))
        }
        helper(&preorder, &mut pre_idx, 0, (inorder.len() as i32) - 1, &mp)
    }
}
