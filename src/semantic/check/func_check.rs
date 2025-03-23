use std::rc::Rc;

use crate::{ast::{expr::LOrExpr, func::{FuncCall, FuncDef, FuncType}}, error::Error};

use super::Analyzer;

impl Analyzer {
  /// 函数调用的检查，包括函数是否存在、参数是否符合函数定义。
  /// 由于目前 Base Type 只有 int(i32)，不需要检查对应参数是否类型匹配，
  /// 仅需检查参数数量是否匹配、表达式是否合法。
  pub fn func_call_check(&mut self, func_call: &FuncCall) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;
    
    let scope = self.get_current_scope();

    let mut res_func_def = scope.func_call_check(func_call);
    if res_func_def.is_err() {
      return Err(res_func_def.err().unwrap());
    }
    let mut func_def_ = res_func_def.unwrap();
    
    while func_def_.is_none() {
      let block = self.get_current_block();

      let parent_id_ = block.get_parent_id();
      if parent_id_.is_none() {
        // 已经找遍所有父级 Block 了，函数不存在
        return Err(Error::SemanticError(format!("{} is not defined", func_call.ident)));
      }
      let parent_id = parent_id_.unwrap();
      
      // 进入父级 Block
      let res = self.set_current_block(parent_id);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      res_func_def = self.get_current_scope().func_call_check(func_call);
      if res_func_def.is_err() {
        return Err(res_func_def.err().unwrap());
      }
      func_def_ = res_func_def.unwrap();
    }

    // 恢复当前 Block Id
    let res = self.set_current_block(cur_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let len = func_def_.clone().unwrap().func_fparams.len();
    for i in 0..len {
      let expr = &func_call.func_rparams[i];
      let res = self.expr_check(expr);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }

    Ok(())
  }

  /// 以 FuncDef 为单位进行语义检查。
  pub fn func_def_check(&mut self, func_def: Rc<FuncDef>) -> Result<(), Error> {
    // 获取当前作用域
    let scope = self.get_current_scope();

    // 检查当前作用域能否定义该函数
    let res = scope.func_def(func_def.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    self.func_block_check(func_def)
  }

  /// return 类型是否符合函数定义的检查
  pub fn return_check(&mut self, expr_: &Option<LOrExpr>) -> Result<(), Error> {
    let mut block = self.get_current_block();
    let mut parent_id_ = block.get_parent_id();
    let mut func_def_ = block.func.clone().borrow().clone();

    while func_def_.is_none() && parent_id_.is_some() {
      block = self.get_current_block();
      parent_id_ = block.get_parent_id();
      func_def_ = block.func.clone().borrow().clone();
    }

    if func_def_.is_none() {
      return Err(Error::SemanticError(format!("'return' can't be used outside a function")));
    }

    match &func_def_.unwrap().func_type {
      FuncType::Void => {
        if expr_.is_some() {
          return Err(Error::SemanticError(format!("'return' should return void")));
        }
      },
      FuncType::BType( btype ) => {
        if expr_.is_none() {
          return Err(Error::SemanticError(format!("'return' should return type {}", btype)));
        }

        let res = self.expr_check(expr_.as_ref().unwrap());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
    }
    Ok(())
  }
}