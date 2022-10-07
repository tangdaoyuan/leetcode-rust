#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = Solution::traverse(&head);
        let index = len - n + 1;
        let mut pos = 1;
        let mut prev = &head;
        let mut new_head = Some(Box::new(ListNode::new(-1)));
        let mut new_head_ptr = &mut new_head;
        while prev.is_some() {
            if pos != index {
                new_head_ptr.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(prev.as_ref().unwrap().val)));
                new_head_ptr = &mut new_head_ptr.as_mut().unwrap().next;
            }
            pos += 1;
            prev = &prev.as_ref().unwrap().next;
        }

        new_head.unwrap().next
    }
    pub fn traverse(head: &Option<Box<ListNode>>) -> i32 {
        let mut len = 0;
        let mut prev = head;
        while prev.is_some() {
            len += 1;
            prev = &prev.as_ref().unwrap().next;
        }
        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 2;
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(3)));
        let mut node4 = Some(Box::new(ListNode::new(4)));
        let node5 = Some(Box::new(ListNode::new(5)));
        node4.as_mut().unwrap().next = node5;
        node3.as_mut().unwrap().next = node4;
        node2.as_mut().unwrap().next = node3;
        head.as_mut().unwrap().next = node2;

        let result = Solution::remove_nth_from_end(head, n);

        assert_eq!(Solution::traverse(&result), 4);
    }
}
