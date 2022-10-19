#![allow(dead_code)]

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

struct Solution;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            if node.as_ref().is_none() {
                return;
            }
            dfs(&mut node.as_ref().unwrap().clone().borrow_mut().left);
            dfs(&mut node.as_ref().unwrap().clone().borrow_mut().right);

            let mut tmp = node.as_deref().unwrap().clone().borrow().left.clone();

            if tmp.is_none() {
                return;
            }

            while tmp.as_ref().unwrap().borrow().right.is_some() {
                tmp = tmp.as_ref().unwrap().clone().borrow().right.clone();
            }
            let mut n = node.as_deref().unwrap().clone().borrow_mut();
            tmp.as_ref().unwrap().clone().borrow_mut().right = n.right.clone();

            n.right = n.left.clone();
            n.left = None;
        }
        dfs(root);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut root = create_test_tree1();
        Solution::flatten(&mut root);
        assert_eq!(
            format(root),
            vec![1, -1, 2, -1, 3, -1, 4, -1, 5, -1, 6, -1, -1]
        );
    }

    fn format(result: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let ans = Rc::new(RefCell::new(vec![]));

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: Rc<RefCell<Vec<i32>>>) {
            if node.is_none() {
                ans.clone().borrow_mut().push(-1);
                return;
            }
            ans.clone()
                .borrow_mut()
                .push(node.as_ref().unwrap().clone().try_borrow().unwrap().val);

            dfs(
                node.as_ref()
                    .unwrap()
                    .clone()
                    .try_borrow()
                    .unwrap()
                    .left
                    .clone(),
                ans.clone(),
            );
            dfs(
                node.as_ref()
                    .unwrap()
                    .clone()
                    .try_borrow()
                    .unwrap()
                    .right
                    .clone(),
                ans.clone(),
            );
        }

        dfs(result, ans.clone());

        ans.clone().try_borrow().unwrap().clone()
    }

    fn create_test_tree1() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(create_node(
            1,
            Some(create_node(
                2,
                Some(create_node(3, None, None)),
                Some(create_node(4, None, None)),
            )),
            Some(create_node(5, None, Some(create_node(6, None, None)))),
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
