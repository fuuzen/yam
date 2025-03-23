use std::rc::Rc;

use super::block::Block;
use super::btype::RVal;
use super::func::FuncDef;
use super::{btype::{BType, LVal}, expr::Expr};

#[derive(Debug)]
pub struct ConstDef {
  pub ident: String,
  pub expr: Expr,
}

#[derive(Debug)]
pub struct ConstDecl {
  pub btype: BType,
  pub const_defs: Vec<ConstDef>,

  /// 确定 BType 后，初始化所有的 RVal
  pub rvals: Vec<Rc<RVal>>,
}

impl ConstDecl {
  pub fn new(btype: BType, const_defs: Vec<ConstDef>) -> Self {
    let len = const_defs.len();
    let mut rvals = Vec::new();
    for _ in 0..len {
      let rval = match btype {
        BType::Int => RVal::new_int(),
        BType::Bool => unimplemented!(),
      };
      rvals.push(Rc::new(rval));
    }
    ConstDecl {
      btype,
      const_defs,
      rvals,
    }
  }
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

  /// 确定 BType 后，初始化所有的 RVal
  pub rvals: Vec<Rc<RVal>>,
}

impl VarDecl {
  pub fn new(btype: BType, var_defs: Vec<VarDef>) -> Self {
    let len = var_defs.len();
    let mut rvals = Vec::new();
    for _ in 0..len {
      let rval = match btype {
        BType::Int => RVal::new_int(),
        BType::Bool => unimplemented!(),
      };
      rvals.push(Rc::new(rval));
    }
    VarDecl {
      btype,
      var_defs,
      rvals,
    }
  }
}

#[derive(Debug)]
pub struct Asgn {
  pub lval: LVal,
  pub expr: Expr,
}

#[derive(Debug)]
pub struct IfElse {
  pub cond: Expr,
  pub if_: Box<Stmt>,
  pub else_: Option<Box<Stmt>>,
}

#[derive(Debug)]
pub struct While {
  pub cond: Expr,
  pub body: Box<Stmt>,
}


#[derive(Debug)]
pub enum Stmt {
  FuncDef(Rc<FuncDef>),
  Expr(Option<Expr>),
  ConstDecl(ConstDecl),
  VarDecl(VarDecl),
  Asgn(Asgn),
  Block(Rc<Block>),
  IfElse(IfElse),
  While(While),
  Break,
  Continue,
  Return(Option<Expr>),
}