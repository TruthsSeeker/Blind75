use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };
    
    return dfs(1, &root);
}

fn dfs(depth: i32, node: &Rc<RefCell<TreeNode>>) -> i32 {
    let (mut left_depth, mut right_depth) = (depth, depth);

    if let Some(left) = &node.borrow().left {
        left_depth = dfs(depth + 1, left);
    }
    if let Some(right) = &node.borrow().right {
        right_depth = dfs(depth + 1, right);
    }

    return left_depth.max(right_depth);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_depth() {
        // Test case 1: Empty tree
        let root1 = None;
        assert_eq!(max_depth(root1), 0);
        
        // Test case 2: Tree with a single node
        let root2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(max_depth(root2), 1);
        
        // Test case 3: Tree with multiple nodes
        let mut root3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root3.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root3.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root3.as_mut().unwrap().borrow().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        root3.as_mut().unwrap().borrow().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(max_depth(root3), 3);
    }
}