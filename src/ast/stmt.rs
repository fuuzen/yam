use std::rc::Rc;

use crate::ast::measure::Measure;
use crate::ast::note::Note;
use crate::ast::phrase::Phrase;
use crate::ast::track::Track;

use super::block::Block;
use super::val::RVal;
use super::func::FuncDef;
use super::{val::{BType, LVal}, expr::Expr};

#[derive(Debug)]
pub enum AsgnRVal {
  Expr(Expr),
  Note(Note),
  Measure(Measure),
  Phrase(Phrase),
  Track(Track),
}

#[derive(Debug)]
pub struct ConstDef {
  pub ident: String,
  pub rval: AsgnRVal,
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
      let rval = RVal::new_with_btype(btype.clone());
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
  pub rval_: Option<AsgnRVal>,
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
      let rval = RVal::new_with_btype(btype.clone());
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
  pub rval: AsgnRVal,
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