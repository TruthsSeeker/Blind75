pub mod max_depth;
pub mod same;
pub mod invert_tree;
pub mod max_path_sum;
pub mod level_order;
pub mod serde_tree;
pub mod subtree;
pub mod build_inorder_preorder;
pub mod validate_binary_tree;
pub mod bst_k_smallest;

use std::{cell::RefCell, rc::Rc};

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
      right: None
    }
  }
}