use std::{ cell::RefCell, rc::Rc};

use super::TreeNode;
use super::same::is_same_tree;

pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dfs(root, sub_root)
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if !is_same_tree(node.clone(), sub_root.clone()) {
        let Some(node) = node else {
            return sub_root.is_none();
        };
        
        return dfs(node.borrow().left.clone(), sub_root.clone()) || dfs(node.borrow().right.clone(), sub_root.clone());
    } else {
        return true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::serde_tree::Codec;
    
    #[test]
    fn test_is_subtree() {
        let codec = Codec::new();
        // // Test case 1: root contains sub root
        // let root1_serialized = "S3,S4,S1,N,N,S2,N,N,5,N,N".to_string();
        // let sub_root1_serialized = "S4,S1,N,N,S2,N,N".to_string();
        
        // let root1 = codec.deserialize(root1_serialized);
        // let sub_root1 = codec.deserialize(sub_root1_serialized);
        // assert!(is_subtree(root1, sub_root1));
        
        // // Test case 2: root doesn't contain sub root
        // let root2_serialized = "S3,S4,S1,N,N,S2,S0,N,N,N,5,N,N".to_string();
        // let sub_root2_serialized = "S4,S1,N,N,S2,N,N".to_string();
        
        // let root2 = codec.deserialize(root2_serialized);
        // let sub_root2 = codec.deserialize(sub_root2_serialized);

        // assert!(!is_subtree(root2, sub_root2));

        // Test case 3: long tree with only right children
        // [1,null,1,null,1,null,1,null,1,null,1,null,1,null,1,null,1,null,1,null,1,2]
        let root3_serialized = "S1,N,S1,N,S1,N,S1,N,S1,N,S1,N,S1,N,S1,N,S1,N,S1,N,S1,S2,N,N,N".to_string();
        let root3 = codec.deserialize(root3_serialized);
        // [1,null,1,null,1,null,1,null,1,null,1,2]
        let sub_root3_serialized = "S1,N,S1,N,S1,N,S1,N,S1,N,S1,S2,N,N,N".to_string();
        let sub_root3 = codec.deserialize(sub_root3_serialized);

        assert!(is_subtree(root3, sub_root3));
    
    }
}