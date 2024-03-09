use super::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n:i32) -> Option<Box<ListNode>> {
    let mut temp = Some(Box::new(ListNode { val: 0, next: head}));
    let mut slow = &mut temp;
    let mut fast = &slow.clone();
    
    for _ in 0..=n {
        fast = &fast.as_ref().unwrap().next;
    }

    while fast.is_some() {
        slow = &mut slow.as_mut().unwrap().next;
        fast = &fast.as_ref().unwrap().next;
    }

    let next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
    slow.as_mut().unwrap().next = next;

    temp.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let mut head = Some(Box::new(ListNode { val: 1, next: None }));
        head = remove_nth_from_end(head, 1);
        assert_eq!(head, None);

        let mut head = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: None })) }));
        head = remove_nth_from_end(head, 1);
        assert_eq!(head, Some(Box::new(ListNode { val: 1, next: None })));

        let mut head = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: None })) })) }));
        head = remove_nth_from_end(head, 2);
        assert_eq!(head, Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: None })) })));
    }
}