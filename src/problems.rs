pub mod lec1;
pub mod lec2;
pub mod lec3;
pub mod lec4;

use std::cell::RefCell;
use std::rc::Rc;

pub use crate::problems::lec1::*;
pub use crate::problems::lec2::*;
pub use crate::problems::lec3::*;
pub use crate::problems::lec4::*;

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
