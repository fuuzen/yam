use std::{cell::RefCell, fmt, rc::Rc};

/// Int 类型的默认初始值
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

  /// 语义检查阶段，绑定该左值的右值
  pub rval: Rc<RefCell<Option<Rc<RVal>>>>,
}

impl LVal {
  pub fn new(ident: String) -> Self {
    LVal {
      ident,
      rval: Rc::new(RefCell::new(None)),
    }
  }

  /// 具有类型 BType 的初始化，初始化为默认值
  pub fn new_with_btype(btype: BType, ident: String) -> Self {
    let rval = match btype {
      BType::Int => RVal::new_int(),
      BType::Bool => unimplemented!(),
    };
    LVal {
      ident,
      rval: Rc::new(RefCell::new(Some(Rc::new(rval)))),
    }
  }

  /// 语义检查阶段绑定右值
  pub fn bind_rval(&self, rval: Rc<RVal>) {
    *self.rval.borrow_mut() = Some(rval);
  }

  /// 执行阶段获取 int 值
  pub fn get_int(&self) -> i32 {
    self.rval.borrow().as_ref().unwrap().get_int()
  }

  /// 赋值 int
  pub fn set_int(&self, value: i32) {
    self.rval.borrow().as_ref().unwrap().set_int(value);
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

  /// 赋值 int
  pub fn set_int(&self, value: i32) {
    match self {
      RVal::Int(rval) => *rval.borrow_mut() = value,
    }
  }

  /// 返回变量类型 Btype
  pub fn get_btype(&self) -> BType {
    match self {
      RVal::Int(_) => BType::Int,
    }
  }

  /// 获取 int 值
  pub fn get_int(&self) -> i32 {
    match self {
      RVal::Int(rval) => *rval.borrow(),
    }
  }
}