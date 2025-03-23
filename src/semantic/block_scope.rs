use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::btype::{LVal, RVal};
use crate::ast::func::{FuncCall, FuncDef};
use crate::error::Error;

use super::symbol::Symbol;

#[derive(Clone)]
pub struct BlockScope {
  symbol_table: Rc<RefCell<HashMap<String, Symbol>>>,
}

impl BlockScope {
  pub fn new() -> Self {
    Self {
      symbol_table: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  /// 声明一个常量或变量。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名常量。
  pub fn decl(&self, ident: &String, const_: bool, rval: Rc<RVal>) -> Result<(), Error> {
    let mut t = self.symbol_table.borrow_mut();
    if t.get(ident).is_none() {
      t.insert(
        ident.clone(),
        Symbol::new_val(const_, rval)
      );
      Ok(())
    } else {
      Err(Error::SemanticError(format!("symbol {} is already defined in this scope", ident)))
    }
  }

  /// 声明并定义一个函数。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名函数
  pub fn func_def(&self, func_def: Rc<FuncDef>) -> Result<(), Error> {
    let mut t = self.symbol_table.borrow_mut();
    if t.get(&func_def.ident).is_none() {
      t.insert(
        func_def.ident.clone(),
        Symbol::new_func(func_def)
      );
      Ok(())
    } else {
      Err(Error::SemanticError(format!("symbol {} is already defined in this scope", func_def.ident)))
    }
  }

  /// 检查对一个变量 LVal 的赋值是否合法。若合法则绑定该 LVal 的 RVal。
  /// 若这一级 Block 中不存在该变量的符号，返回值为假，上层 Block 还需要继续检查。
  /// 由于目前 Base Type 只有 int(i32)，不需要赋值的类型检查。
  /// 类型检查放在表达式的检查中，即检查表达式结果类型。
  pub fn asgn_check(&self, lval: &LVal) -> Result<bool, Error> {
    let ident = match lval {
      LVal {ident, ..} => ident
    };
    let t = self.symbol_table.borrow();
    let symbol_ = t.get(ident);
    if symbol_.is_some() {
      if symbol_.unwrap().func_def.is_some() {
        Err(Error::SemanticError(format!("cannot assign to function {}", symbol_.unwrap().func_def.as_ref().unwrap().ident)))
      } else {
        if symbol_.unwrap().const_ {
          Err(Error::SemanticError(format!("cannot assign to constant {}", *ident)))
        } else {
          lval.bind_rval(symbol_.unwrap().rval.clone().unwrap());
          Ok(true)
        }
      }
    } else {
      Ok(false)
    }
  }

  /// 变量或常量调用的检查。若合法则绑定该 LVal 的 RVal。
  /// 若这一级 Block 中不存在该变量或常量的符号，返回值为 None，上层 Block 还需要继续检查。
  /// 若存在该 Lval 的符号，返回相应的 Symbol 中的 Rc<RVal>。
  pub fn lval_check(&self, lval: &LVal) -> Result<Option<Rc<RVal>>, Error> {
    let ident = match lval {
      LVal {ident, ..} => ident,
    };
    let k = ident.clone();
    let t = self.symbol_table.borrow();
    let symbol_ = t.get(&k);
    if symbol_.is_none() {
      Ok(None)
    } else if symbol_.unwrap().func_def.is_some() {
      Err(Error::SemanticError(format!("{} is a function at this scope", *ident)))
    } else {
      lval.bind_rval(symbol_.unwrap().rval.clone().unwrap());
      Ok(Some(symbol_.unwrap().rval.clone().unwrap()))
    }
  }

  /// 检查对一个函数的调用是否合法。
  /// 若这一级 Block 中不存在该函数的符号，返回值为 None，上层 Block 还需要继续检查。
  /// 若存在该函数的符号，绑定 FuncDef 给 FuncCall，返回相应的 Symbol 中的 Rc<FuncDef>。
  /// 由于目前 Base Type 只有 int(i32)，检查调用参数是否匹配仅需检查参数数量是否匹配。
  pub fn func_call_check(&self, func_call: &FuncCall) -> Result<Option<Rc<FuncDef>>, Error> {
    let (ident, func_rparams) = match func_call {
      FuncCall{ident, func_rparams, ..} => (ident, func_rparams),
    };
    let k = ident.clone();
    let t = self.symbol_table.borrow();
    let symbol_ = t.get(&k);
    if symbol_.is_none() {
      Ok(None)
    } else if symbol_.unwrap().func_def.as_ref().unwrap().func_fparams.len() != func_rparams.len() {
      Err(Error::SemanticError(format!("params not match when calling function {}", *ident)))
    } else {
      func_call.bind_func_def(symbol_.unwrap().func_def.clone().unwrap());
      Ok(Some(symbol_.unwrap().func_def.clone().unwrap()))
    }
  }
}
