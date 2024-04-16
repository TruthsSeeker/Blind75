use std::{cell::RefCell, rc::Rc};
use super::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_sub(root, i64::MIN, i64::MAX)
}

fn is_valid_sub(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    let Some(root) = root else {
        return true;
    };
    let val = i64::from(root.borrow().val);
    if val <= min || val >= max {
        return false;
    }
    
    return 
        is_valid_sub(root.borrow().left.clone(), min, val) 
        && is_valid_sub(root.borrow().right.clone(), val, max)
}

#[cfg(test)]
mod tests {
    use crate::trees::serde_tree::Codec;
    use super::*;


    #[test]
    fn test_valid_bst() {
        let codec = Codec::new();

        // Valid BST
        let tree1 = "S2,S1,N,N,S3,N,N".to_string();
        let root1 = codec.deserialize(tree1);
        assert!(is_valid_bst(root1));

        // Invalid BST 1
        let tree2 = "S5,S1,N,N,S4,S3,N,N,S6,N,N".to_string();
        let root2 = codec.deserialize(tree2);
        assert!(!is_valid_bst(root2));

        // Invalid BST 2
        let tree3 = "S5,S4,N,N,S6,S3,N,N,S7,N,N".to_string();
        let root3 = codec.deserialize(tree3);
        assert!(!is_valid_bst(root3));
    }
}