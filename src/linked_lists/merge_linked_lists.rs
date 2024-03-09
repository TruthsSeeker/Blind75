use std::collections::VecDeque;

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

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 { return None; }
    let mut lists = lists;
    let mut new_head = lists.pop()?;
    if lists.len() < 1 {
        return new_head;
    }
    while let Some(other_list) = lists.pop() {
        new_head = merge_two_lists(new_head, other_list);
    }

    new_head
}

pub fn merge_k_list_dac(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = VecDeque::from(lists);
    while lists.len() > 1 {
        let mut merged_lists = VecDeque::new();
        while let (Some(l1), l2_opt) = (lists.pop_front(), lists.pop_front()) {
            let l2 = match l2_opt {
                Some(l2) => l2,
                None => None,
            };
            merged_lists.push_back(merge_two_lists(l1, l2))
        } 
        lists = merged_lists;
    }
    
    lists.pop_front()?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // [[1,4,5],[1,3,4],[2,6]]
        let input = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None,
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: None,
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: None,
                })),
            })),
        ];
        // [1,1,2,3,4,4,5,6]
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 6,
                                        next: None,
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_k_lists(input), expected);
    }

    #[test]
    fn test_2() {
        // [[1], [0]]
        let input = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
            Some(Box::new(ListNode {
                val: 0,
                next: None,
            })),
        ];
        // [0,1]
        let expected = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
        }));
        assert_eq!(merge_k_lists(input), expected);
    }

    #[test]
    fn test_dac_1() {
        // [[1,4,5],[1,3,4],[2,6]]
        let input = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None,
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: None,
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: None,
                })),
            })),
        ];
        // [1,1,2,3,4,4,5,6]
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 6,
                                        next: None,
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(merge_k_list_dac(input), expected);
    }

    #[test]
    fn test_dac_2() {
        // [[1], [0]]
        let input = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
            Some(Box::new(ListNode {
                val: 0,
                next: None,
            })),
        ];
        // [0,1]
        let expected = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: None,
            })),
        }));
        assert_eq!(merge_k_list_dac(input), expected);
    }
}