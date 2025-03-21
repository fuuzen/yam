use std::{cell::RefCell, fmt, rc::Rc};

/// 每一种 Base Type 的默认初始值
pub const INT_DEFAULT: i32 = 0;

/// 所有的 Base Type 的定义
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BType {
  Int,
  Bool,
}

impl fmt::Display for BType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BType::Int => write!(f, "int"),
      BType::Bool => write!(f, "bool"),
    }
  }
}

/// Left Value，左值
#[derive(Debug, Clone)]
pub struct LVal {
  pub ident: String,

  /// 语义检查阶段，若存在该左值的定义，给出其定义
  pub rval: Rc<RefCell<Option<RVal>>>,
}

impl LVal {
  pub fn new(ident: String) -> Self {
    LVal {
      ident,
      rval: Rc::new(RefCell::new(None)),
    }
  }

  pub fn set_rval(&self, rval: RVal) {
    *self.rval.borrow_mut() = Some(rval);
  }
}

/// Rightt Value，右值，每一种 Base Type 的具体存储类型。
/// parse 阶段没有使用，语义检查阶段创建
#[derive(Debug, Clone)]
pub enum RVal {
  Int(Rc<RefCell<i32>>),
}

impl RVal {
  /// 语义检查阶段统一初始化为默认值；实际运行时视初始化为普通的赋值。
  pub fn new_int() -> Self {
    RVal::Int(Rc::new(RefCell::new(INT_DEFAULT)))
  }
}