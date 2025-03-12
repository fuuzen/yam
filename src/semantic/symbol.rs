use std::rc::Rc;

use crate::ast::{btype::BType, func::FuncDef};


#[derive()]
pub struct Symbol {
  pub const_: bool,
  pub btype: Option<BType>,
  pub func_def: Option<Rc<FuncDef>>,
}

impl Symbol {
  /// 从函数定义 AST FuncDef 新建一个函数符号
  pub fn new_func(func_def: Rc<FuncDef>) -> Self {
    Self {
      const_: true,
      btype: None,
      func_def: Some(func_def),
    }
  }

  /// 从 Base Type 新建一个常量或变量
  pub fn from_btype(btype: &BType, const_: bool) -> Self {
    Self {
      const_,
      btype: Some(btype.clone()),
      func_def: None,
    }
  }
}
