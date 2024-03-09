use std::collections::VecDeque;

use super::ListNode;

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut head = head;
    let mut dequeue = VecDeque::new();
    
    let mut first = head.as_mut().unwrap().next.take();
    while let Some(mut node) = first {
        first = node.next.take();
        dequeue.push_back(Some(node));
    }

    while !dequeue.is_empty() {
        if let Some(back) = dequeue.pop_back() {
            head.as_mut().unwrap().next = back;
            head = &mut head.as_mut().unwrap().next;
        }
        if let Some(front) = dequeue.pop_front() {
            head.as_mut().unwrap().next = front;
            head = &mut head.as_mut().unwrap().next;
        }
    }
}