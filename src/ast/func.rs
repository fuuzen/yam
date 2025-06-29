use std::{cell::RefCell, rc::Rc};

use crate::ast::val::Value;

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
      let rval = RVal::new_with_btype(btype.clone());
    FuncFParam {
      ident,
      rval: Rc::new(rval),
    }
  }

  /// 赋值
  pub fn set_value(&self, value: Value) {
    self.rval.set_value(value);
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