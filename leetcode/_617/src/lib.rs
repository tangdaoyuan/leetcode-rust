#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        
        fn dfs(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>) {
            if node2.is_none() {
                return;
            }
            let n1 = node1.as_ref().unwrap().clone();
            let n2 = node2.as_ref().unwrap().clone();
            let a = node1.as_ref().unwrap().clone().borrow().val;
            let b = n2.as_ref().borrow().val;
            n1.borrow_mut().val = a + b;


            if n1.borrow().left.is_none() {
                n1.borrow_mut().left =  node2.as_ref().unwrap().borrow().left.clone();
            } else {
                dfs(node1.as_ref().unwrap().borrow().left.clone(), node2.as_ref().unwrap().borrow().left.clone())
            }

            if n1.borrow().right.is_none() {
                n1.borrow_mut().right = node2.as_ref().unwrap().borrow().right.clone();
            } else {
                dfs(node1.unwrap().borrow().right.clone(), node2.unwrap().borrow().right.clone())
            }
        }

        dfs(root1.clone(), root2.clone());
        root1.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root1 = create_test_tree1();
        let root2 = create_test_tree2();

        let result = Solution::merge_trees(root1, root2);
        assert_eq!(format(result.clone()), vec![3, 4, 5, -1, -1, 4, -1, -1, 5, -1, 7, -1, -1]);
    }

    fn format(result: Option<Rc<RefCell<TreeNode>>> ) -> Vec<i32> {
        let ans = Rc::new(RefCell::new(vec![]));

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: Rc<RefCell<Vec<i32>>>) {
            if node.is_none() {
                ans.clone().borrow_mut().push(-1);
                return
            }
            ans.clone().borrow_mut().push(node.as_ref().unwrap().clone().try_borrow().unwrap().val);

            dfs(node.as_ref().unwrap().clone().try_borrow().unwrap().left.clone(), ans.clone());
            dfs(node.as_ref().unwrap().clone().try_borrow().unwrap().right.clone(), ans.clone());
        }

        dfs(result, ans.clone());

        ans.clone().try_borrow().unwrap().clone()
    }

    fn create_test_tree1() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(create_node(
            1,
            Some(create_node(3, Some(create_node(5, None, None)), None)),
            Some(create_node(2, None, None)),
        ))))
    }

    fn create_test_tree2() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(create_node(
            2,
            Some(create_node(1, None, Some(create_node(4, None, None)))),
            Some(create_node(3, None, Some(create_node(7, None, None)))),
        ))))
    }

    fn create_result_tree() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(create_node(
            3,
            Some(create_node(
                4,
                Some(create_node(5, None, None)),
                Some(create_node(4, None, None)),
            )),
            Some(create_node(5, None, Some(create_node(7, None, None)))),
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
