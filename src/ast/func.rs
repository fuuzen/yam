use std::{cell::RefCell, rc::Rc};

use super::{btype::BType, expr::Expr, block::Block};

#[derive(Debug)]
pub enum FuncType {
  BType(BType),
  Void,
}

#[derive(Debug)]
pub struct FuncFParam {
  pub btype: BType,
  pub ident: String, 
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