pub type IntConst = i32;
pub type Exp = LOrExp;

#[derive(Debug)]
pub struct CompUnit {
  pub tempo: IntConst,
  pub time_signature: TimeSignature,
  pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {  // return type
  Midi,
  Mp3,
}

#[derive(Debug)]
pub struct Block {
  pub stmts: Vec<Play>,
}

#[derive(Debug)]
pub struct TimeSignature {
  pub numerator: IntConst,
  pub denominator: IntConst,
}

#[derive(Debug)]
pub struct Play {
  pub bar: IntConst,
  pub start: IntConst,
  pub end: IntConst,
  pub pitch: IntConst,
}

#[derive(Debug)]
pub enum UnaryOp {
  Plus,
  Minus,
  Not,
}

#[derive(Debug)]
pub enum MulOp {
  Mul,
  Div,
  Mod,
}

#[derive(Debug)]
pub enum AddOp {
  Add,
  Sub,
}

#[derive(Debug)]
pub enum RelOp {
  Gt,
  Lt,
  Ge,
  Le,
}

#[derive(Debug)]
pub enum EqOp {
  Eq,
  Ne,
}

#[derive(Debug)]
pub enum PrimaryExp {
  Exp(Exp),
  LVal(String),
  Number(IntConst),
}

#[derive(Debug)]
pub struct UnaryExp {
  pub unary_ops: Vec<UnaryOp>,
  pub primary_exp: PrimaryExp,
}

#[derive(Debug)]
pub struct MulExp {
  pub mul_ops: Vec<MulOp>,
  pub unary_exps: Vec<UnaryExp>,
}

#[derive(Debug)]
pub struct AddExp {
  pub add_ops: Vec<AddOp>,
  pub mul_exps: Vec<MulExp>,
}

#[derive(Debug)]
pub struct RelExp {
  pub rel_ops: Vec<RelOp>,
  pub add_exps: Vec<AddExp>,
}

#[derive(Debug)]
pub struct EqExp {
  pub eq_ops: Vec<EqOp>,
  pub rel_exps: Vec<RelExp>,
}

#[derive(Debug)]
pub struct LAndExp {
  pub eq_exps: Vec<EqExp>,
}

#[derive(Debug)]
pub struct LOrExp {
  pub land_exps: Vec<LAndExp>,
}
