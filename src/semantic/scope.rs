use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::btype::{BType, LVal};
use crate::ast::expr::Expr;
use crate::ast::func::FuncDef;
use crate::error::Error;

use super::symbol::Symbol;


pub struct BlockScope {
  symbol_table: HashMap<String, Symbol>,
}

impl BlockScope {
  pub fn new() -> Self {
    Self {
      symbol_table: HashMap::new(),
    }
  }

  /// 声明一个常量或变量。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名常量。
  pub fn decl(&mut self, btype: &BType, lval: &LVal, const_: bool) -> Result<(), Error> {
    let ident = match lval {
      LVal::Ident(ident) => ident.clone()
    };
    if self.symbol_table.get(&ident).is_none() {
      self.symbol_table.insert(
        ident,
        Symbol::from_btype(btype, const_)
      );
      Ok(())
    } else {
      let err = format!("symbol {} is already defined in this scope", ident);
      Err(Error::SemanticError(err))
    }
  }

  /// 声明并定义一个函数
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名函数
  pub fn func_def(&mut self, func_def: Rc<FuncDef>) -> Result<(), Error> {
    if self.symbol_table.get(&func_def.ident).is_none() {
      self.symbol_table.insert(
        func_def.ident.clone(),
        Symbol::new_func(func_def)
      );
      Ok(())
    } else {
      let err = format!("symbol {} is already defined in this scope", func_def.ident);
      Err(Error::SemanticError(err))
    }
  }

  /// 检查对一个变量 LVal 的赋值是否合法。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名常量和变量，
  /// 若这一级 Block 中不存在该变量的符号，返回值为假，上层 Block 还需要继续检查。
  /// 由于目前 Base Type 只有 int(i32)，不需要赋值的类型检查。
  /// 类型检查放在表达式的检查中，即检查表达式结果类型。
  pub fn asgn_check(&self, lval: &LVal) -> Result<bool, Error> {
    let err: String;
    let ident = match lval {
      LVal::Ident(ident) => ident
    };
    let symbol_ = self.symbol_table.get(ident);
    if symbol_.is_some() {
      if symbol_.unwrap().func_def.is_some() {
        err = format!("cannot assign to function {}", symbol_.unwrap().func_def.as_ref().unwrap().ident);
        Err(Error::SemanticError(err))
      } else {
        if symbol_.unwrap().const_ {
          err = format!("cannot assign to constant {}", *ident);
          Err(Error::SemanticError(err))
        } else {
          Ok(true)
        }
      }
    } else {
      Ok(false)
    }
  }

  /// 检查对一个函数的调用是否合法。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名函数，
  /// 若这一级 Block 中不存在该函数的符号，返回值为假，上层 Block 还需要继续检查。
  /// 由于目前 Base Type 只有 int(i32)，检查调用参数是否匹配仅需检查参数数量是否匹配。
  pub fn func_call_check(&self, ident: &String, func_rparams: Vec<Expr>) -> Result<bool, Error> {
    let k = ident.clone();
    let symbol_ = self.symbol_table.get(&k);
    if symbol_.is_none() {
      Ok(false)
    } else if symbol_.unwrap().func_def.as_ref().unwrap().func_fparams.len() != func_rparams.len() {
      let err = format!("params not match when calling function {}", *ident);
      Err(Error::SemanticError(err))
    } else {
      Ok(true)
    }
  }
}