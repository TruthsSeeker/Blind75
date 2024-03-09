pub mod reverse;
pub mod merge_linked_lists;


// Definition for singly-linked list.
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