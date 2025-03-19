use std::rc::Rc;
use std::hash::{Hash, Hasher};

use crate::ast::btype::RVal;
use crate::ast::{block::BlockId, btype::BType, func::FuncDef};

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

  /// 用于在翻译阶段的哈希表中和所有作用域的符号区分。
  pub block_id: BlockId,

  /// 若为 Base Type，则存储具体的值；
  /// 若为函数，则该值无意义。
  pub rval: Option<RVal>,
}

/// 实现 PartialEq trait，不比较 func_def
impl PartialEq for Symbol {
  fn eq(&self, other: &Self) -> bool {
    self.const_ == other.const_
      && self.const_ == other.const_
      && self.btype == other.btype
      && self.block_id == other.block_id
  }
}

/// 实现 Eq trait
impl Eq for Symbol {}

/// 实现 Hash trait
impl Hash for Symbol {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.const_.hash(state);
    self.btype.hash(state);
    self.block_id.hash(state);
  }
}

impl Symbol {
  /// 从函数定义 AST FuncDef 新建一个函数符号
  pub fn new_func(func_def: Rc<FuncDef>, block_id: BlockId) -> Self {
    Self {
      const_: true,
      btype: None,
      func_def: Some(func_def),
      block_id,
      rval: None,
    }
  }

  /// 从 Base Type 新建一个常量或变量
  pub fn new_val(btype: &BType, const_: bool, block_id: BlockId) -> Self {
    Self {
      const_,
      btype: Some(btype.clone()),
      func_def: None,
      block_id,
      rval: Some(RVal::new_int()),
    }
  }
}
