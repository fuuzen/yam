use std::rc::Rc;

use crate::ast::btype::RVal;
use crate::ast::{btype::BType, func::FuncDef};

/// 表示符号的数据结构，在语义分析时给出，代表唯一的一个符号
#[derive(Clone)]
pub struct Symbol {
  /// 若为 Base Type，则表示其是否为常量；
  /// 若为函数，则该值无意义。
  pub const_: bool,

  /// 若为 Base Type，则存储对应的类型；
  /// 若为函数，则该值无意义。
  pub btype: Option<BType>,

  /// 若为函数，则存储对应函数定义 AST 的引用；
  /// 若为 Base Type 数据，则该值无意义。
  pub func_def: Option<Rc<FuncDef>>,

  /// 若为 Base Type，则存储具体的值；
  /// 若为函数，则该值无意义。
  pub rval: Option<Rc<RVal>>,
}

impl Symbol {
  /// 从函数定义 AST FuncDef 新建一个函数符号
  pub fn new_func(func_def: Rc<FuncDef>) -> Self {
    Self {
      const_: true,
      btype: None,
      func_def: Some(func_def),
      rval: None,
    }
  }

  /// 从 Base Type 新建一个常量或变量
  pub fn new_val(btype: &BType, const_: bool, rval: Rc<RVal>) -> Self {
    Self {
      const_,
      btype: Some(btype.clone()),
      func_def: None,
      rval: Some(rval),
    }
  }
}
