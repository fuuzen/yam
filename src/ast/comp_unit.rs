use std::rc::Rc;

use crate::ast::score::Score;

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
  pub block: Rc<Block>,
  pub score: Score
}