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
  pub block: Rc<RefCell<Block>>,
}

#[derive(Debug)]
pub struct FuncCall {
  pub ident: String,
  pub func_rparams: Vec<Expr>,
}