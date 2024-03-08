#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[allow(dead_code)]
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = None;

    // take mutable reference so that changes to the tail are reflected in the new_head
    let mut tail = &mut new_head;
    let mut current1 = list1;
    let mut current2 = list2;

    loop {
        // unwrap the current node from the first list, if it doesn't exist the rest of the second list is already sorted
        let mut l1 = match current1 {
            Some(l1) => l1,
            None => { 
                *tail = current2;
                break;
            }
        };
        // unwrap the current node from the second list, if it doesn't exist the rest of the first list is already sorted
        let mut l2 = match current2 {
            Some(l2) => l2,
            None => {
                *tail = Some(l1);
                break;
            },
        };

        if l1.val < l2.val {
            // take() the value from l1.next so that tail.next is None after the assignment
            current1 = l1.next.take();
            current2 = Some(l2);
            *tail = Some(l1);
        } else {
            current2 = l2.next.take();
            current1 = Some(l1);
            *tail = Some(l2);
        }
        // move the tail to the next node so we can assign the next node to it in the next iteration
        // unwrap is safe because we always assign Some to tail
        tail = &mut tail.as_mut().unwrap().next;
    }

    new_head
}