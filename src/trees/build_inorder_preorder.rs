use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = preorder.first()?;
    let inorder_i = inorder.iter().position(|v| v == root)?;

    let inorder_ls = inorder[..inorder_i].to_vec();
    let preorder_ls = preorder[1..=inorder_ls.len()].to_vec();
    
    let inorder_rs = inorder[1 + inorder_i..].to_vec();
    let preorder_rs = preorder[inorder_ls.len() + 1..].to_vec();

    let node = TreeNode {
        val: *root,
        left: build_tree(preorder_ls, inorder_ls),
        right: build_tree(preorder_rs, inorder_rs)
    };
    return Some(Rc::new(RefCell::new(node)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::serde_tree::Codec;
    
    #[test]
    fn test_build_tree() {
        // In-order
        // [1,2,3,4,5,6,7,8,9]
        // Pre-order
        // [6,2,1,4,3,5,7,9,8]
        let tree_result_expected = "S6,S2,S1,N,N,S4,S3,N,N,S5,N,N,S7,N,S9,S8,N,N,N".to_string();

        let root = build_tree(vec![6,2,1,4,3,5,7,9,8], vec![1,2,3,4,5,6,7,8,9]);
        let codec = Codec::new();
        let serialized = codec.serialize(root);
        assert_eq!(tree_result_expected, serialized);
    }
}