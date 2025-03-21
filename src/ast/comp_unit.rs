use std::rc::Rc;

use super::func::FuncDef;
use super::block::Block;
use super::stmt::{ConstDecl, VarDecl};

#[derive(Debug)]
pub enum Def {
  ConstDecl(ConstDecl),
  VarDecl(VarDecl),
  FuncDef(Rc<FuncDef>),
}

#[derive(Debug)]
pub struct CompUnit {
  pub defs: Option<Vec<Def>>,
  pub ident: String, 
  pub block: Rc<Block>,
}