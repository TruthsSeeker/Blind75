use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut queue = VecDeque::new();
    let (rootp, rootq ) = (p, q);
    queue.push_back((rootp.clone(), rootq.clone()));
    while let Some((node_p, node_q)) = queue.pop_front() {
        if node_p.is_none() && node_q.is_none() {
            continue;
        }

        let (Some(node_p), Some(node_q)) = (node_p, node_q) else {
            return false;
        };

        if node_p.borrow().val != node_q.borrow().val {
            return false;
        }
    
        queue.push_back((node_p.borrow().left.clone(), node_q.borrow().left.clone()));
        queue.push_back((node_p.borrow().right.clone(), node_q.borrow().right.clone()));

    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_same_tree() {
        // Test case 1: Both trees are empty
        assert_eq!(is_same_tree(None, None), true);
        
        // Test case 2: One tree is empty, the other is not
        let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(is_same_tree(tree1.clone(), None), false);
        
        // Test case 3: Both trees have the same structure and values
        let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let tree3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(is_same_tree(tree2.clone(), tree3.clone()), true);
        
        // Test case 5: Both trees have the same structure but different values
        let tree6 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let tree7 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(is_same_tree(tree6.clone(), tree7.clone()), false);
    }
}