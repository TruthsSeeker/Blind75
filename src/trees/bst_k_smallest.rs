use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let (_, val) = inorder_dfs(root, k, 0);
    val
}


fn inorder_dfs(root: Option<Rc<RefCell<TreeNode>>>, k: i32, n: i32) -> (i32, i32) {
    let Some(root) = root else {
        return (n, -1);
    };
    
    let (mut n, val) = inorder_dfs(root.borrow().left.clone(), k, n);
    
    if val != -1 {
        return (k, val); 
    }

    n += 1;
    if k == n {
        return (n, root.borrow().val);
    }

    return inorder_dfs(root.borrow().right.clone(), k, n);
}

#[cfg(test)]
mod tests {
    use crate::trees::serde_tree::Codec;

    use super::*;
    
    #[test]
    fn test_kth_smallest() {
        let codec = Codec::new();
        let tree = "S3,S1,N,S2,N,N,S4,N,N".to_string();
        let root = codec.deserialize(tree);
        assert_eq!(kth_smallest(root, 1), 1);

        let tree = "S5,S3,S2,S1,N,N,N,S4,N,N,S6,N,N".to_string();
        let root = codec.deserialize(tree);
        assert_eq!(kth_smallest(root, 3), 3);
    }
}