#![allow(dead_code)]

struct Solution;

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
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![];
        let mut ptr = root;
        while ptr.is_some() || !stack.is_empty() {
            while ptr.is_some() {
                let node = ptr.unwrap();
                stack.push(Some(node.clone()));

                ptr = match node.as_ref().borrow().left {
                    Some(_) => Some(node.borrow().left.as_ref().unwrap().clone()),
                    None => None,
                };
            }
            ptr = stack.pop().unwrap();
            ans.push(ptr.as_ref().unwrap().borrow().val);
            let node = match ptr.as_ref().unwrap().borrow().right {
                Some(_) => Some(
                    ptr.as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .clone(),
                ),
                None => None,
            };
            ptr = node;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::inorder_traversal(create_tree());
        assert_eq!(result, vec![1, 3, 2]);
    }

    fn create_tree() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(create_node(
            1,
            None,
            Some(create_node(2, Some(create_node(3, None, None)), None)),
        ))))
    }

    fn create_node(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> TreeNode {
        TreeNode {
            val,
            left: match left {
                None => None,
                _ => Some(Rc::new(RefCell::new(left.unwrap()))),
            },
            right: match right {
                None => None,
                _ => Some(Rc::new(RefCell::new(right.unwrap()))),
            },
        }
    }
}
