use std::rc::Rc;
use std::cell::RefCell;
use super::TreeNode;
pub struct Codec {
	
}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = vec![];
        Self::ser_dfs(root, &mut res);
        res.iter().map(|n| {
            match n {
                Some(v) => format!("S{}", v),
                None => "N".to_string(),
            }
        }).reduce(|acc, s| {
            let mut joined = acc;
            joined.push_str(",");
            joined.push_str(s.as_str());
            return joined
        }).unwrap_or("".to_string())
        
    }

    fn ser_dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Option<i32>>) {
        match node {
            Some(val) => {
                res.push(Some(val.borrow().val));
                Self::ser_dfs(val.borrow().left.clone(), res);
                Self::ser_dfs(val.borrow().right.clone(), res);
            },
            None => res.push(None),
        }
    }
	
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let split = data.split(",").fuse();
        let mut nodes = vec![];
        for n in split {
            nodes.push(match n {
                v if v.starts_with("S") => { v[1..].parse::<i32>().ok()},
                _ => None,
            })
        }

        Self::de_dfs(0, &nodes).0
    }

    fn de_dfs(i: usize, nodes: &Vec<Option<i32>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        let Some(val) = nodes[i] else {
            return (None, i + 1);
        };
        let mut i = i + 1;
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        (node.borrow_mut().left, i) = Self::de_dfs(i, nodes);
        (node.borrow_mut().right, i) = Self::de_dfs(i, nodes);

        (Some(node), i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let codec = Codec::new();

        // Create a binary tree
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let node4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let node5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        root.as_ref().unwrap().borrow_mut().left = node2.clone();
        root.as_ref().unwrap().borrow_mut().right = node3.clone();
        node3.as_ref().unwrap().borrow_mut().left = node4.clone();
        node3.as_ref().unwrap().borrow_mut().right = node5.clone();

        let expected = "S1,S2,N,N,S3,S4,N,N,S5,N,N";
        assert_eq!(codec.serialize(root), expected);
    }

    #[test]
    fn test_deserialize() {
        let codec = Codec::new();
        let data = "S1,S2,N,N,S3,S4,N,N,S5,N,N".to_string();
        let root = codec.deserialize(data);

        // Verify the deserialized tree structure
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
        assert_eq!(root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val, 2);
        assert_eq!(root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().val, 3);
        assert_eq!(root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val, 4);
        assert_eq!(root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().val, 5);
    }
}