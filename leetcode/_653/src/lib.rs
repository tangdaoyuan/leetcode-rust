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
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
pub struct Solution;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut _set = HashSet::new();
        return Solution.dfs(&root, k, &mut _set);
    }
    pub fn dfs(
        &self,
        node: &Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        _set: &mut HashSet<i32>,
    ) -> bool {
        match node {
            Some(rc) => {
                if _set.contains(&(k - rc.borrow().val)) {
                    return true;
                }
                let val = rc.borrow().val;
                _set.insert(val);

                self.dfs(&rc.borrow().left, k, _set) || self.dfs(&rc.borrow().right, k, _set)
            }
            _ => false,
        }
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
        let mut node5 = TreeNode::new(5);
        let mut node3 = TreeNode::new(3);
        let mut node6 = TreeNode::new(6);
        let node2 = TreeNode::new(2);
        let node4 = TreeNode::new(4);
        let node7 = TreeNode::new(7);

        node3.left = Some(Rc::new(RefCell::new(node2)));
        node3.right = Some(Rc::new(RefCell::new(node4)));
        node6.right = Some(Rc::new(RefCell::new(node7)));
        node5.left = Some(Rc::new(RefCell::new(node3)));
        node5.right = Some(Rc::new(RefCell::new(node6)));
        let root = Some(Rc::new(RefCell::new(node5)));
        let result = Solution::find_target(root, 9);

        assert_eq!(result, true);
    }
}
