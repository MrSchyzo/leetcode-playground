use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
}

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        root.as_ref().map(|node| Self::explore(node, k)).unwrap().0
    }

    fn explore(node: &Rc<RefCell<TreeNode>>, k: i32) -> (i32, i32) {
        let me = node.borrow();
        let (l_val, l_idx) = me.left.as_ref()
            .map(|l| Self::explore(l, k))
            .unwrap_or((me.val, k));
        let (val, idx) = (me.val, l_idx-1);

        return match (me.right.as_ref(), idx) {
            (Some(r), x) if x >= 1 => Self::explore(r, idx),
            (_, -1) => (l_val, 0),
            (_, x) => (val, x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: some more realistic tests, perhaps?
    #[test]
    fn call_on_singleton_node_returns_that_node() {
        let set = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(Solution::kth_smallest(set, 0), 1);
    }

    #[test]
    fn call_on_singleton_node_returns_that_node_index_1() {
        let set = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(Solution::kth_smallest(set, 1), 1);
    }

    #[test]
    fn call_on_singleton_node_returns_that_node_regardless() {
        let set = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(Solution::kth_smallest(set, -11), 1);
    }

    #[test]
    fn call_on_singleton_node_returns_that_node_regardless_2() {
        let set = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(Solution::kth_smallest(set, 11), 1);
    }
}
