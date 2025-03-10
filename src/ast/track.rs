use super::func::FuncDef;
use super::stmt::{ConstDecl, VarDecl, Block};

#[derive(Debug)]
pub struct Track {
  pub const_decls: Vec<ConstDecl>,
  pub var_decls: Vec<VarDecl>,
  pub func_defs: Vec<FuncDef>,
  pub block: Block,
}