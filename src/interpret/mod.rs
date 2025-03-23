pub mod calc; /// 表达式计算
pub mod ctr;  /// 控制流

// use midi_file::MidiFile;
// use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
// use midi_file::file::{QuartersPerMinute, Track};

use std::{collections::HashMap, rc::Rc};

use ctr::{Ctr, RetVal};

use crate::ast::expr::Expr;
use crate::ast::stmt::{Asgn, ConstDecl, IfElse, Stmt, VarDecl, While};
use crate::error::Error;

use crate::{ast::{block::{Block, BlockId}, func::FuncCall, comp_unit::CompUnit}, semantic::{blocks::Blocks, scope::{BlockScope, Scopes}}};

/// 解释器
pub struct Interpreter {
  /// 指向当前分析检查到的 Block 的 Id
  current_block_id: BlockId,

  /// 根据 BlockId 快速定位到 Rc<Block>
  block_table: HashMap<BlockId, Rc<Block>>,

  /// 根据 BlockId 快速定位到 BlockScope
  scope_table: HashMap<BlockId, BlockScope>,
}

impl Interpreter {
  pub fn new(blocks: Blocks, scopes: Scopes) -> Self {
    Self {
      current_block_id: 0,
      block_table: blocks.get_block_table().clone(),
      scope_table: scopes.get_scope_table().clone(),
    }
  }

  /// 设置当前的 Block 的 Id
  pub fn set_current_block(&mut self, block_id: BlockId) {
    self.current_block_id = block_id;
  }

  /// 根据给定 BlockId 获取 Rc<Block>
  pub fn get_block(&self, block_id: BlockId) -> Result<Rc<Block>, Error> {
    Ok(self.block_table.get(&block_id).ok_or_else(|| Error::RuntimeError(format!("can't get this block: {}", block_id))).unwrap().clone())
  }

  /// 根据给定 BlockId 获取 BlockScope
  pub fn get_scope(&self, block_id: BlockId) -> Result<BlockScope, Error> {
    Ok(self.scope_table.get(&block_id).ok_or_else(|| Error::RuntimeError(format!("can't get BlockScope of this block: {}", block_id))).unwrap().clone())
  }

  /// 执行一段函数，返回结果为 RetVal 类型
  pub fn call_func(&mut self, func_call: &FuncCall) -> Result<RetVal, Error> {
    let func_def = func_call.get_func_def();
    let len = func_def.func_fparams.len();
    for i in 0..len {
      let param = &func_def.func_fparams[i];
      let expr = &func_call.func_rparams[i];
      let res = self.calc_expr(expr);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      param.set_int(res.unwrap());
    }
    
    self.set_current_block(func_def.block.block_id);
    let res = self.intpret_block(func_def.block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    match res.unwrap() {
      Ctr::Return( v ) => Ok(v),
      _ => Err(Error::RuntimeError("function didn't return".to_string())),
    }
  }

  pub fn intpret_const_decl(&mut self, const_decl: &ConstDecl) -> Result<Ctr, Error> {
    let len = const_decl.const_defs.len();
    for i in 0..len {
      let const_def = &const_decl.const_defs[i];
      let res = self.calc_expr(&const_def.expr);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      let rval = const_decl.rvals[i].clone();
      rval.set_int(res.unwrap());
    }
    Ok(Ctr::None)
  }

  pub fn interpret_var_decl(&mut self, var_decl: &VarDecl) -> Result<Ctr, Error> {
    let len = var_decl.var_defs.len();
    for i in 0..len {
      let var_def = &var_decl.var_defs[i];
      let expr_ = var_def.expr_.as_ref();
      if expr_.is_none() {
        continue;
      }

      let res = self.calc_expr(expr_.unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      let rval = var_decl.rvals[i].clone();
      rval.set_int(res.unwrap());
    }
    Ok(Ctr::None)
  }

  pub fn intpret_asgn(&mut self, asgn: &Asgn) -> Result<Ctr, Error> {
    let res = self.calc_expr(&asgn.expr);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    let lval = asgn.lval.clone();
    lval.set_int(res.unwrap());
    Ok(Ctr::None)
  }

  pub fn intpret_ifelse(&self, ifelse: &IfElse) -> Result<Ctr, Error> {
    Ok(Ctr::None)
  }

  pub fn intpret_while(&mut self, while_: &While) -> Result<Ctr, Error> {
    let res = self.calc_expr(&while_.cond);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    while res.clone().unwrap() != 0 {
      let res = self.intpret_stmt(&while_.body);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      match res.clone().unwrap() {
        Ctr::Break => return Ok(Ctr::None), // while 循环结束
        Ctr::Return( _ ) => return res,
        _ => (),
      }

      let res = self.calc_expr(&while_.cond);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(Ctr::None)
  }

  pub fn intpret_return(&mut self, expr_: &Option<Expr>) -> Result<Ctr, Error> {
    if expr_.is_some() {
      let res = self.calc_expr(expr_.as_ref().unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      Ok(Ctr::Return( RetVal::Int(res.unwrap()) ))
    } else {
      Ok(Ctr::Return( RetVal::Void ))
    }
  }

  pub fn intpret_stmt(&mut self, stmt: &Stmt) -> Result<Ctr, Error> {
    match stmt {
      Stmt::Expr( expr_ ) => {
        if expr_.is_some() {
          let res = self.calc_expr(expr_.as_ref().unwrap());
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        }
        Ok(Ctr::None)
      },
      Stmt::ConstDecl( const_decl ) => self.intpret_const_decl(const_decl),
      Stmt::VarDecl( var_decl ) => self.interpret_var_decl(var_decl),
      Stmt::Asgn( asgn ) => self.intpret_asgn(asgn),
      Stmt::Block( block ) => self.intpret_block(block.clone()),
      Stmt::IfElse( if_ ) => self.intpret_ifelse(if_),
      Stmt::While( while_ ) => self.intpret_while(while_),
      Stmt::Break => Ok(Ctr::Break),
      Stmt::Continue => Ok(Ctr::Continue),
      Stmt::Return( expr_ ) => self.intpret_return(expr_),
      _ => Ok(Ctr::None),
    }
  }

  pub fn intpret_block(&mut self, block: Rc<Block>) -> Result<Ctr, Error> {
    for stmt in &block.stmts {
      let res = self.intpret_stmt(stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      match res.clone().unwrap() {
        Ctr::Break => return res,
        Ctr::Return( _ ) => return res,
        _ => (),
      }
    }
    Ok(Ctr::None)
  }

  pub fn interpret(&mut self, comp_unit: &CompUnit) -> Result<(), Error> {
    let res = self.intpret_block(comp_unit.block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }
}