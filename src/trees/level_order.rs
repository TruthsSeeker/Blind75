use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut results = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((0, root));

    while !queue.is_empty() {
        let Some((lvl, Some(node))) = queue.pop_front() else {
            continue;
        };
        queue.push_back((lvl + 1, node.borrow().left.clone()));
        queue.push_back((lvl + 1, node.borrow().right.clone()));
        if lvl == results.len() {
            results.push(vec![node.borrow().val])
        } else {
            results[lvl].push(node.borrow().val);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_level_order() {
        // Create a binary tree for testing
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        let right_left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let right_right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        
        root.as_ref().unwrap().borrow_mut().left = left.clone();
        root.as_ref().unwrap().borrow_mut().right = right.clone();
        right.as_ref().unwrap().borrow_mut().left = right_left.clone();
        right.as_ref().unwrap().borrow_mut().right = right_right.clone();
        
        // Call the function under test
        let result = level_order(root);
        
        // Check the expected output
        let expected = vec![
            vec![3],
            vec![9, 20],
            vec![15, 7],
        ];
        assert_eq!(result, expected);
    }
}
