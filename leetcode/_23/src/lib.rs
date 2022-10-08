#![allow(dead_code)]

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Status {
    val: i32,
    ptr: Box<ListNode>,
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue: BinaryHeap<Status> = BinaryHeap::new();
        for node in lists {
            if node.is_some() {
                queue.push(Status {
                    val: node.as_ref().unwrap().val,
                    ptr: node.unwrap(),
                })
            }
        }

        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        while !(queue.is_empty()) {
            let f = queue.pop();
            tail.as_mut().unwrap().next = Some(f.as_ref().unwrap().ptr.clone());
            tail = &mut tail.as_mut().unwrap().next;
            if f.as_ref().unwrap().ptr.as_ref().next.is_some() {
                queue.push(Status {
                    val: f.as_ref().unwrap().ptr.as_ref().next.as_ref().unwrap().val,
                    ptr: f
                        .as_ref()
                        .unwrap()
                        .ptr
                        .as_ref()
                        .next
                        .as_ref()
                        .unwrap()
                        .clone(),
                });
            }
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut node1 = Some(Box::new(ListNode::new(1)));
        let mut node1_1 = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(3)));
        let mut node4 = Some(Box::new(ListNode::new(4)));
        let node4_1 = Some(Box::new(ListNode::new(4)));
        let node5 = Some(Box::new(ListNode::new(5)));
        let node6 = Some(Box::new(ListNode::new(6)));
        node4.as_mut().unwrap().next = node5;
        node1.as_mut().unwrap().next = node4;
        node3.as_mut().unwrap().next = node4_1;
        node1_1.as_mut().unwrap().next = node3;
        node2.as_mut().unwrap().next = node6;

        let lists = vec![node1, node1_1, node2];
        let result = Solution::merge_k_lists(lists);
        assert_ne!(result, None);
    }
}
