use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    while let Some(node) = queue.pop_front() {
        let Some(node) = node else {
            continue;
        };
        let mut node = node.borrow_mut();
        (node.left, node.right) = (node.right.clone(), node.left.clone());
        queue.push_back(node.left.clone());
        queue.push_back(node.right.clone());
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invert_tree() {
        // Test case 1: Empty tree
        let result = invert_tree(None);
        assert_eq!(result, None);
        
        // Test case 2: Tree with a single node
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = invert_tree(root.clone());
        assert_eq!(result, root);
        
        // Test case 3: Tree with multiple nodes
        let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.as_ref().unwrap().borrow_mut().left = left.clone();
        root.as_ref().unwrap().borrow_mut().right = right.clone();
        let result = invert_tree(root.clone());
        assert_eq!(result.as_ref().unwrap().borrow().val, 4);
        assert_eq!(result.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val, 7);
        assert_eq!(result.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().val, 2);
    }
}