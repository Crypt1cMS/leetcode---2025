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

struct Solution {}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let mut result: i32 = 0;

                if node.val >= low && node.val <= high {
                    result += node.val;
                }

                result += Solution::range_sum_bst(node.left.clone(), low, high);
                result += Solution::range_sum_bst(node.right.clone(), low, high);

                result
            }

            None => 0,
        }
    }
}