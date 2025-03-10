#[derive(Debug)]
pub struct ConstDef {
  pub ident: String,
  pub expr: Expr,
}

#[derive(Debug)]
pub struct ConstDecl {
  pub btype: BType,
  pub const_defs: Vec<ConstDef>,
}

#[derive(Debug)]
pub struct VarDef {
  pub ident: String,
  pub expr_: Option<Expr>,
}

#[derive(Debug)]
pub struct VarDecl {
  pub btype: BType,
  pub var_defs: Vec<VarDef>,
}

#[derive(Debug)]
pub struct Asgn {
  pub lval: LVal,
  pub expr: Expr,
}

#[derive(Debug)]
pub struct If {
  pub cond: Expr,
  pub then: Stmt,
  pub else_: Option<Stmt>,
}

#[derive(Debug)]
pub struct While {
  pub cond: Expr,
  pub body: Stmt,
}

#[derive(Debug)]
pub enum Stmt {
  ConstDecl(ConstDecl),
  VarDecl(VarDecl),
  Asgn(Asgn),
  Block(Block),
  If(If),
  While(While),
  Break,
  Continue,
  Return(Option<Expr>),
}

#[derive(Debug)]
pub struct Block {
  pub stmts: Vec<Stmt>,
}