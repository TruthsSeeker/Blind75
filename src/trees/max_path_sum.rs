use std::{cell::RefCell, rc::Rc};

use super::TreeNode;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
        return 0
    };
    let mut max = root.clone().borrow_mut().val;
    dfs(Some(root), &mut max);
    max
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    let Some(node) = node else { return 0 };
    let left = dfs(node.borrow_mut().left.clone(), max);
    let right = dfs(node.borrow_mut().right.clone(), max);
    let (left_max, right_max) = (left.max(0), right.max(0));
    
    *max = *max.max(&mut (node.borrow_mut().val + left_max + right_max));

    return node.borrow().val + left_max.max(right_max);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_path_sum() {
        // Test case 1: Single node tree
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(max_path_sum(root), 1);
        
        // Test case 2: Tree with negative values
        let root = Rc::new(RefCell::new(TreeNode::new(-10)));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root.as_ref().borrow_mut().left = left.clone();
        root.as_ref().borrow_mut().right = right.clone();
        right.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(max_path_sum(Some(root)), 42);
        
        // Test case 3: Tree with positive values
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let mut left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let left_right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let right_left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let right_right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.as_ref().borrow_mut().left = left.clone();
        root.as_ref().borrow_mut().right = right.clone();
        left.as_mut().unwrap().borrow_mut().left = left_left;
        left.unwrap().as_ref().borrow_mut().right = left_right;
        right.as_mut().unwrap().borrow_mut().left = right_left;
        right.as_ref().unwrap().borrow_mut().right = right_right;
        assert_eq!(max_path_sum(Some(root)), 18);
    }
}