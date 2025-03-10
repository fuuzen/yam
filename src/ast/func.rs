use super::btype::BType;

#[derive(Debug)]
pub enum FuncType {
  BType(BType),
  Void,
}

#[derive(Debug)]
pub struct FuncFParam {
  pub btype: Btype,
  pub ident: String, 
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub func_fparams: Vec<FuncRParam>,
  pub block: Block,
}

pub struct FuncCall {
  pub ident: String,
  pub func_rparams: Vec<Expr>,
}