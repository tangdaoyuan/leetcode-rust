#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            prefix: i32,
            map: &mut HashMap<i32, i32>,
        ) -> i32 {
            if node.is_none() {
                return 0;
            }
            let mut count = 0;
            let node = node.as_ref().unwrap().borrow();
            let cur = prefix + node.val;

            count = *map.get(&(cur - target_sum)).or(Some(&0)).unwrap();
            map.insert(cur, map.get(&cur).or(Some(&0)).unwrap() + 1);
            count += dfs(&node.left, target_sum, cur, map);
            count += dfs(&node.right, target_sum, cur, map);
            map.insert(cur, map.get(&cur).or(Some(&0)).unwrap() - 1);
            return count;
        }

        return dfs(&root, target_sum, 0, &mut map);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node3_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: -2,
            left: None,
            right: None,
        })));
        let node_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: -2,
            left: None,
            right: None,
        })));
        let node11 = Some(Rc::new(RefCell::new(TreeNode {
            val: 11,
            left: None,
            right: None,
        })));

        node3.as_ref().unwrap().borrow_mut().right = node11;
        node3.as_ref().unwrap().borrow_mut().left = node3_2;
        node3.as_ref().unwrap().borrow_mut().right = node_2;
        node2.as_ref().unwrap().borrow_mut().right = node1;
        node5.as_ref().unwrap().borrow_mut().left = node3;
        node5.as_ref().unwrap().borrow_mut().right = node2;

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: node5,
            right: node_3,
        })));

        let target = 8;

        let result = Solution::path_sum(root, target);
        assert_eq!(result, 3);
    }
}
