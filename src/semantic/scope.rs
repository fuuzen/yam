use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::block::BlockId;
use crate::ast::btype::{BType, LVal, RVal};
use crate::ast::func::{FuncCall, FuncDef};
use crate::ast::stmt::Stmt;
use crate::error::Error;

use super::symbol::Symbol;

#[derive(Clone)]
pub struct BlockScope {
  symbol_table: HashMap<String, Symbol>,

  /// 当前 Block 内当前分析检查到的 Stmt 前有几个连续出现的 Stmt::If
  /// 初始值为 0，当前 Stmt 若为 Stmt::If 则加 1，若为 Stmt::Else 则减 1，若都不是则归 0。
  /// 若为负数则表示出现了 if 和 else 不匹配的情况。
  if_else_stack: i32,
}

impl BlockScope {
  pub fn new() -> Self {
    Self {
      symbol_table: HashMap::new(),
      if_else_stack: 0,
    }
  }

  /// 按规则设置 if_else_stack，并检查其是否变为负数
  pub fn if_else_check(&mut self, stmt: &Stmt) -> Result<(), Error> {
    match stmt {
      Stmt::If(_) => self.if_else_stack += 1,
      Stmt::Else(_) => self.if_else_stack -= 1,
      _ => self.if_else_stack = 0,
    }
    match self.if_else_stack < 0 {
      true => Err(Error::SemanticError(format!("'else' can't pair with any 'if'"))),
      false => Ok(())
    }
  }

  /// 声明一个常量或变量。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名常量。
  pub fn decl(&mut self, btype: &BType, ident: &String, const_: bool, block_id: &BlockId, rval: Rc<RVal>) -> Result<(), Error> {
    if self.symbol_table.get(ident).is_none() {
      self.symbol_table.insert(
        ident.clone(),
        Symbol::new_val(btype, const_, *block_id, rval)
      );
      Ok(())
    } else {
      Err(Error::SemanticError(format!("symbol {} is already defined in this scope", ident)))
    }
  }

  /// 声明并定义一个函数。
  /// 作用域检查仅限于本 Block，故可以遮蔽上层 Block 的同名函数
  pub fn func_def(&mut self, func_def: Rc<FuncDef>, block_id: &BlockId) -> Result<(), Error> {
    if self.symbol_table.get(&func_def.ident).is_none() {
      self.symbol_table.insert(
        func_def.ident.clone(),
        Symbol::new_func(func_def, *block_id)
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
    let symbol_ = self.symbol_table.get(ident);
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
    let symbol_ = self.symbol_table.get(&k);
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
    let symbol_ = self.symbol_table.get(&k);
    if symbol_.is_none() {
      Ok(None)
    } else if symbol_.unwrap().func_def.as_ref().unwrap().func_fparams.len() != func_rparams.len() {
      Err(Error::SemanticError(format!("params not match when calling function {}", *ident)))
    } else {
      func_call.bind_func_def(symbol_.unwrap().func_def.clone().unwrap());
      Ok(Some(symbol_.unwrap().func_def.clone().unwrap()))
    }
  }

  /// 尝试根据 identity 获取一个符号的 Option
  pub fn get_symbol(&self, ident: &String) -> Option<&Symbol> {
    self.symbol_table.get(ident)
  }
}

pub struct Scopes {
  /// 所有的 scope，用哈希表存储
  scope_table: HashMap<BlockId, BlockScope>,
}

impl Scopes {
  pub fn new() -> Self {
    Self {
      scope_table: HashMap::new(),
    }
  }

  /// 在哈希表中创建一个给定 BlockId 的新的 BlockScope
  pub fn add_scope(&mut self, block_id: BlockId) -> Result<(), Error> {
    let res = self.scope_table.insert(block_id, BlockScope::new());
    if res.is_some() {
      return Err(Error::InternalError(format!("block id conflict: {}", block_id)));
    }
    Ok(())
  }

  /// 获取给定 BlockId 的父级 scope
  pub fn get_parent_scope(&self, block_id: &BlockId) -> Result<& BlockScope, Error> {
    self.scope_table.get(block_id).ok_or(Error::InternalError(format!("can't find parent block for this block {}", block_id)))
  }

  /// 获取给定 BlockId 的 &mut BlockScope
  pub fn get_scope(&mut self, block_id: &BlockId) -> Result<&mut BlockScope, Error> {
    self.scope_table.get_mut(block_id).ok_or(Error::InternalError(format!("can't find the scope of this block: {}", block_id)))
  }

  /// 获取整个 scope_table，用于交给解释器执行
  pub fn get_scope_table(&self) -> &HashMap<BlockId, BlockScope> {
    &self.scope_table
  }
}