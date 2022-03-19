#![allow(overflowing_literals)]
#![allow(dead_code)]
#![allow(unused_assignments)]
// 606.根据二叉树创建字符串

use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let ret_str = Solution::pre_traverse(root.as_ref());
        return ret_str;
    }

    pub fn pre_traverse(node: Option<&Rc<RefCell<TreeNode>>>) -> String {
        if node.is_some() {
            let current = node.as_ref().unwrap().borrow();
            let mut str = current.val.to_string();
            if current.left.is_some() || current.right.is_some() {
                let left = current.left.as_ref();
                str = format!("{}({})", str, Solution::pre_traverse(left));
            }
            if current.right.is_some() {
                let right = current.right.as_ref();
                str = format!("{}({})", str, Solution::pre_traverse(right));
            }

            return str;
        }

        return String::from("");
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_works() {
        // let str = String::from("");
        let mut node1 = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);

        node2.left = Some(Rc::new(RefCell::new(node4)));
        node1.left = Some(Rc::new(RefCell::new(node2)));
        node1.right = Some(Rc::new(RefCell::new(node3)));

        let root = Some(Rc::new(RefCell::new(node1)));
        let str = Solution::tree2str(root);

        assert_eq!(str, "1(2(4))(3)");
    }
}
