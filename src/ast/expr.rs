pub type IntConst = i32;
pub type Expr = LOrExpr;

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
pub enum PrimaryExpr {
  Expr(Expr),
  LVal(String),
  Number(IntConst),
}

#[derive(Debug)]
pub struct UnaryExpr {
  pub unary_ops: Vec<UnaryOp>,
  pub primary_exp: PrimaryExpr,
}

#[derive(Debug)]
pub struct MulExpr {
  pub mul_ops: Vec<MulOp>,
  pub unary_exps: Vec<UnaryExpr>,
}

#[derive(Debug)]
pub struct AddExpr {
  pub add_ops: Vec<AddOp>,
  pub mul_exps: Vec<MulExpr>,
}

#[derive(Debug)]
pub struct RelExpr {
  pub rel_ops: Vec<RelOp>,
  pub add_exps: Vec<AddExpr>,
}

#[derive(Debug)]
pub struct EqExpr {
  pub eq_ops: Vec<EqOp>,
  pub rel_exps: Vec<RelExpr>,
}

#[derive(Debug)]
pub struct LAndExpr {
  pub eq_exps: Vec<EqExpr>,
}

#[derive(Debug)]
pub struct LOrExpr {
  pub land_exps: Vec<LAndExpr>,
}
