use super::func::FuncDef;
use super::block::Block;
use super::stmt::{ConstDecl, VarDecl};

#[derive(Debug)]
pub enum Def {
  ConstDecl(ConstDecl),
  VarDecl(VarDecl),
  Func(FuncDef),
}

#[derive(Debug)]
pub struct Track {
  pub defs: Option<Vec<Def>>,
  pub ident: String, 
  pub block: Block,
}