pub mod calc; /// 表达式计算
pub mod ctr;  /// 控制流
pub mod asgn_rval;  /// 翻译右值表达式

// use midi_file::MidiFile;
// use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
// use midi_file::file::{QuartersPerMinute, Track};

use std:: rc::Rc;

use ctr::{Ctr, RetVal};

use crate::ast::expr::Expr;
use crate::ast::stmt::{Asgn, ConstDecl, IfElse, Stmt, VarDecl, While};
use crate::ast::val::Value;
use crate::error::Error;

use crate::ast::{block::Block, func::FuncCall, comp_unit::CompUnit};

/// 解释器
pub struct Interpreter {}

impl Interpreter {
  pub fn new() -> Self {
    Self {}
  }

  /// 执行一段函数，返回结果为 RetVal 类型
  pub fn call_func(&mut self, func_call: &FuncCall) -> Result<RetVal, Error> {
    let func_def = func_call.get_func_def();
    let len = func_def.func_fparams.len();
    for i in 0..len {
      let param = &func_def.func_fparams[i];
      let asgn_rval = &func_call.func_rparams[i];
      let res = self.interpret_asgn_rval(asgn_rval);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      match res.unwrap() {
        RetVal::Value( v ) => param.set_value(v),
        val => {
          let ident = &func_def.func_fparams[i].ident;
          let expect_type = param.rval.get_btype();
          let func_name = &func_def.ident;
          return Err(Error::RuntimeError(format!(
            "can not use {val} as param {ident}({expect_type}) when calling '{func_name}'"
          )))
        }
      }
    }
    
    let res = self.interpret_block(func_def.block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    match res.unwrap() {
      Ctr::Return( v ) => Ok(v),
      _ => Err(Error::RuntimeError("function didn't return".to_string())),
    }
  }

  pub fn interpret_const_decl(&mut self, const_decl: &ConstDecl) -> Result<Ctr, Error> {
    let len = const_decl.const_defs.len();
    for i in 0..len {
      let res = self.interpret_asgn_rval(&const_decl.const_defs[i].rval);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      match res.unwrap() {
        RetVal::Value( v ) => const_decl.rvals[i].set_value(v),
        val => {
          let ident = &const_decl.const_defs[i].ident;
          let expect_type = const_decl.rvals[i].get_btype();
          return Err(Error::RuntimeError(format!(
            "can not asign {val} to {ident}({expect_type})"
          )))
        }
      }
    }
    Ok(Ctr::None)
  }

  pub fn interpret_var_decl(&mut self, var_decl: &VarDecl) -> Result<Ctr, Error> {
    let len = var_decl.var_defs.len();
    for i in 0..len {
      let var_def = &var_decl.var_defs[i];
      let asgn_rval_ = var_def.rval_.as_ref();
      if asgn_rval_.is_none() {
        continue;
      }

      let res = self.interpret_asgn_rval(asgn_rval_.as_ref().unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      match res.unwrap() {
        RetVal::Value( v ) => var_decl.rvals[i].set_value(v),
        val => {
          let ident = &var_decl.var_defs[i].ident;
          let expect_type = var_decl.rvals[i].get_btype();
          return Err(Error::RuntimeError(format!(
            "can not asign {val} to {ident}({expect_type})"
          )))
        }
      }
    }
    Ok(Ctr::None)
  }

  pub fn interpret_asgn(&mut self, asgn: &Asgn) -> Result<Ctr, Error> {
    let res = self.interpret_asgn_rval(&asgn.rval);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    match res.unwrap() {
      RetVal::Value( v ) => asgn.lval.set_value(v),
      val => {
        let ident = asgn.lval.clone().ident;
        let expect_type = asgn.lval.rval.borrow().as_ref().unwrap().get_btype();
        return Err(Error::RuntimeError(format!(
          "can not asign {val} to {ident}({expect_type})"
        )))
      }
    }
    Ok(Ctr::None)
  }

  pub fn interpret_ifelse(&mut self, ifelse: &IfElse) -> Result<Ctr, Error> {
    let res = self.calc_expr(&ifelse.cond);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    match res.clone().unwrap() {
      RetVal::Value(Value::Int(0)) => match &ifelse.else_ {
        Some( else_ ) => self.interpret_stmt(else_),
        None => Ok(Ctr::None),
      },
      RetVal::Value(Value::Int(_)) => self.interpret_stmt(&ifelse.if_),
      val => Err(Error::RuntimeError(format!(
        "expect int/bool for condition, but found {val}"
      )))
    }
  }

  pub fn interpret_while(&mut self, while_: &While) -> Result<Ctr, Error> {
    let mut res_cond = self.calc_expr(&while_.cond);
    if res_cond.is_err() {
      return Err(res_cond.err().unwrap());
    }

    while res_cond.clone().is_ok_and( |v| match v {
      RetVal::Value(Value::Int(0)) => false,
      RetVal::Value(Value::Int(_)) => true,
      _ => false,
    }){
      let res = self.interpret_stmt(&while_.body);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      match res {
        Ok(Ctr::Break) => return Ok(Ctr::None), // while 循环结束
        Ok(Ctr::Return( _ )) => return res,
        _ => (),
      }

      res_cond = self.calc_expr(&while_.cond);
      if res_cond.is_err() {
        return Err(res.err().unwrap());
      }
    }
    
    match res_cond.clone().unwrap() {
      RetVal::Value(Value::Int(_)) => Ok(Ctr::None),
      val => Err(Error::RuntimeError(format!(
        "expect int/bool for condition, but found {val}"
      )))
    }
  }

  pub fn interpret_return(&mut self, expr_: &Option<Expr>) -> Result<Ctr, Error> {
    if expr_.is_some() {
      let res = self.calc_expr(expr_.as_ref().unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      Ok(Ctr::Return( res.unwrap() ))
    } else {
      Ok(Ctr::Return( RetVal::Void ))
    }
  }

  pub fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Ctr, Error> {
    match stmt {
      Stmt::FuncDef( _ ) => Ok(Ctr::None),
      Stmt::Break => Ok(Ctr::Break),
      Stmt::Continue => Ok(Ctr::Continue),
      Stmt::ConstDecl( const_decl ) => self.interpret_const_decl(const_decl),
      Stmt::VarDecl( var_decl ) => self.interpret_var_decl(var_decl),
      Stmt::Asgn( asgn ) => self.interpret_asgn(asgn),
      Stmt::Block( block ) => self.interpret_block(block.clone()),
      Stmt::IfElse( if_ ) => self.interpret_ifelse(if_),
      Stmt::While( while_ ) => self.interpret_while(while_),
      Stmt::Return( expr_ ) => self.interpret_return(expr_),
      Stmt::Expr( expr_ ) => match expr_.is_some() {
        true => self.calc_expr(expr_.as_ref().unwrap()).map(|_| { Ctr::None }),
        false => Ok(Ctr::None),
      },
    }
  }

  pub fn interpret_block(&mut self, block: Rc<Block>) -> Result<Ctr, Error> {
    for stmt in &block.stmts {
      let res = self.interpret_stmt(stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      match res {
        Ok(Ctr::Break) => return res,
        Ok(Ctr::Return( _ )) => return res,
        _ => (),
      }
    }
    Ok(Ctr::None)
  }

  pub fn interpret(&mut self, comp_unit: &CompUnit) -> Result<RetVal, Error> {
    for stmt in &comp_unit.block.stmts {
      match stmt {
        Stmt::VarDecl( var_decl ) => {
          let res = self.interpret_var_decl(var_decl);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        },
        Stmt::ConstDecl( const_decl ) => {
          let res = self.interpret_const_decl(const_decl);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        },
        _ => (),
      }
    }
    let block = comp_unit.score.block.clone();
    let res = self.interpret_block(block);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    Ok(RetVal::Void)
  }
}