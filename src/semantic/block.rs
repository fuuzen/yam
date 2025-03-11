use std::collections::HashMap;

use crate::ast::btype::{BType, RVal, LVal, INT_DEFAULT};
use crate::ast::expr::Expr;
use crate::error::Error;

pub type BlockId = u32;

/// 追踪 Block 作用域内所有声明的函数、常量、变量的值的 Block 的状态
pub struct BlockState {
  func_table: HashMap<String, BlockId>,
  const_table: HashMap<(BType, LVal), RVal>,
  var_table: HashMap<(BType, LVal), RVal>,
}

impl BlockState {
  pub fn new() -> Self {
    Self {
      func_table: HashMap::new(),
      const_table: HashMap::new(),
      var_table: HashMap::new(),
    }
  }

  /// 声明一个常量
  pub fn const_decl(&mut self, btype: &BType, lval: &LVal, rval: &RVal) -> Result<(), Error> {
    let k = (btype.clone(), lval.clone());
    let v = rval.clone();
    if self.const_table.get(&k).is_none() {
      self.const_table.insert(k, v);
      Ok(())
    } else {
      let err: String;
      match lval {
        LVal::Ident(ident) => {
          err = format!("redeclaration of constant {}", *ident);
        }
      }
      Err(Error::ParseError(err))
    }
  }

  /// 声明一个变量，若没有指定初始值，则使用 yam::ast::btypes 所定义的默认初始值
  pub fn var_decl(&mut self, btype: &BType, lval: &LVal, rval: Option<RVal>) -> Result<(), Error> {
    let k = (btype.clone(), lval.clone());
    if self.var_table.get(&k).is_none() {
      if rval.is_none() {
        self.var_table.insert(k, INT_DEFAULT);
      } else {
        let v = rval.unwrap().clone();
        self.var_table.insert(k, v);
      }
      Ok(())
    } else {
      let err: String;
      match lval {
        LVal::Ident(ident) => {
          err = format!("redeclaration of variant {}", *ident);
        }
      }
      Err(Error::ParseError(err))
    }
  }

  /// 声明并定义一个函数
  pub fn func_def(&mut self, ident: &String, block_id: &BlockId) -> Result<(), Error> {
    let k = ident.clone();
    let v = block_id.clone();
    if self.func_table.get(&k).is_none() {
      self.func_table.insert(k, v);
      Ok(())
    } else {
      Err(Error::ParseError(format!("redeclaration of function {}", *ident)))
    }
  }

  /// 检查对一个变量 LVal 的赋值是否合法
  pub fn asgn_check(&self, btype: &BType, lval: &LVal) -> Result<(), Error> {
    let err: String;
    let k = (btype.clone(), lval.clone());
    if self.const_table.get(&k).is_some() {
      match lval {
        LVal::Ident(ident) => {
          err = format!("cannot assign to constant {}", *ident);
        }
      }
      Err(Error::ParseError(err))
    } else if self.var_table.get(&k).is_none() {
      match lval {
        LVal::Ident(ident) => {
          err = format!("variable {} is not defined", *ident);
        }
      }
      Err(Error::ParseError(err))
    } else {
      Ok(())
    }
  }

  /// 检查对一个函数的调用是否合法
  /// TODO: 调用参数的类型检查
  pub fn func_call_check(&self, ident: &String, _func_rparams: Vec<Expr>) -> Result<(), Error> {
    let k = ident.clone();
    if self.func_table.get(&k).is_none() {
      Err(Error::ParseError(format!("function {} is not defined", *ident)))
    } else {
      Ok(())
    }
  }
}