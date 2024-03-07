// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Box<ListNode> = head?.clone();
    let Some(mut curr) = prev.next else {
        return Some(prev);
    };
    prev.next = None;
    
    while let Some(next) = curr.next {
        curr.next = Some(prev);
        prev = curr;
        curr = next;
    }
    curr.next = Some(prev);

    Some(curr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(reverse_list(input), expected);
    }
}
