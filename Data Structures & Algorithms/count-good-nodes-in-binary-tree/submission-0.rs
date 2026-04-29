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

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        fn dfs(curr: Rc<RefCell<TreeNode>>, last_val: i32, count: &mut i32) {
            let curr_val = curr.borrow().val;
            if last_val <= curr_val {
                *count += 1;
            }
            let last_val = curr_val.max(last_val);
            if let Some(l) = &curr.borrow().left {
                dfs(l.clone(), last_val, count);
            }
            if let Some(r) = &curr.borrow().right {
                dfs(r.clone(), last_val, count);
            }
        }
        let mut count = 0;
        dfs(root, -101, &mut count);
        count
    }
}
