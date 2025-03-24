use std::{cell::RefCell, rc::Rc};

use super::{block::Block, val::{BType, RVal}, expr::Expr};

#[derive(Debug)]
pub enum FuncType {
  BType(BType),
  Void,
}

/// 相当于在 parse 阶段就绑定好 RVal 为其类型的默认值的 LVal。
#[derive(Debug, Clone)]
pub struct FuncFParam {
  pub ident: String,
  pub rval: Rc<RVal>,
}

impl FuncFParam {
  /// 初始化为类型 BType 默认值
  pub fn new(btype: BType, ident: String) -> Self {
    let rval = match btype {
      BType::Int => RVal::new_int(),
      BType::Bool => unimplemented!(),
    };
    FuncFParam {
      ident,
      rval: Rc::new(rval),
    }
  }

  /// 获取 BType
  pub fn get_btype(&self) -> BType {
    self.rval.get_btype()
  }

  /// 执行阶段获取 int 值
  pub fn get_int(&self) -> i32 {
    self.rval.get_int()
  }

  /// 赋值 int
  pub fn set_int(&self, value: i32) {
    self.rval.set_int(value);
  }
}


#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub func_fparams: Vec<FuncFParam>,
  pub block: Rc<Block>,
}

#[derive(Debug)]
pub struct FuncCall {
  pub ident: String,
  pub func_rparams: Vec<Expr>,

  /// 语义检查阶段绑定的函数定义
  pub func_def: Rc<RefCell<Option<Rc<FuncDef>>>>,
}

impl FuncCall {
  pub fn new(ident: String, func_rparams: Vec<Expr>) -> Self {
    FuncCall {
      ident,
      func_rparams,
      func_def: Rc::new(RefCell::new(None)),
    }
  }

  pub fn bind_func_def(&self, func_def: Rc<FuncDef>) {
    *self.func_def.borrow_mut() = Some(func_def);
  }

  /// 获取绑定的 FuncDef
  pub fn get_func_def(&self) -> Rc<FuncDef> {
    self.func_def.borrow().as_ref().unwrap().clone()
  }
}