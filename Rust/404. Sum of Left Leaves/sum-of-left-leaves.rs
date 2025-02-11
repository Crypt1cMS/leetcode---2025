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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node_ref = node.borrow();
                let mut sum = 0;

                if let Some(left) = &node_ref.left {
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        sum += left.borrow().val;
                    }
                }

                sum += Solution::sum_of_left_leaves(node_ref.left.clone());
                sum += Solution::sum_of_left_leaves(node_ref.right.clone());

                sum
            }
            None => 0,
        }
    }
}