pub mod calc; /// 表达式计算
pub mod ctr;  /// 控制流

// use midi_file::MidiFile;
// use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
// use midi_file::file::{QuartersPerMinute, Track};

use std:: rc::Rc;

use ctr::{Ctr, RetVal};

use crate::ast::expr::Expr;
use crate::ast::stmt::{Asgn, ConstDecl, IfElse, Stmt, VarDecl, While};
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
      let expr = &func_call.func_rparams[i];
      let res = self.calc_expr(expr);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      param.set_int(res.unwrap());
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

  pub fn interpret_asgn(&mut self, asgn: &Asgn) -> Result<Ctr, Error> {
    let res = self.calc_expr(&asgn.expr);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    let lval = asgn.lval.clone();
    lval.set_int(res.unwrap());
    Ok(Ctr::None)
  }

  pub fn interpret_ifelse(&mut self, ifelse: &IfElse) -> Result<Ctr, Error> {
    let res = self.calc_expr(&ifelse.cond);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    match res.clone().unwrap() {
      0 => match &ifelse.else_ {
        Some( else_ ) => self.interpret_stmt(else_),
        None => Ok(Ctr::None),
      },
      _ => self.interpret_stmt(&ifelse.if_),
    }
  }

  pub fn interpret_while(&mut self, while_: &While) -> Result<Ctr, Error> {
    let mut res_cond = self.calc_expr(&while_.cond);
    if res_cond.is_err() {
      return Err(res_cond.err().unwrap());
    }

    while res_cond.clone().unwrap() != 0 {
      let res = self.interpret_stmt(&while_.body);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      match res.clone().unwrap() {
        Ctr::Break => return Ok(Ctr::None), // while 循环结束
        Ctr::Return( _ ) => return res,
        _ => (),
      }

      res_cond = self.calc_expr(&while_.cond);
      if res_cond.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(Ctr::None)
  }

  pub fn interpret_return(&mut self, expr_: &Option<Expr>) -> Result<Ctr, Error> {
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

  pub fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Ctr, Error> {
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
      Stmt::ConstDecl( const_decl ) => self.interpret_const_decl(const_decl),
      Stmt::VarDecl( var_decl ) => self.interpret_var_decl(var_decl),
      Stmt::Asgn( asgn ) => self.interpret_asgn(asgn),
      Stmt::Block( block ) => self.interpret_block(block.clone()),
      Stmt::IfElse( if_ ) => self.interpret_ifelse(if_),
      Stmt::While( while_ ) => self.interpret_while(while_),
      Stmt::Break => Ok(Ctr::Break),
      Stmt::Continue => Ok(Ctr::Continue),
      Stmt::Return( expr_ ) => self.interpret_return(expr_),
      _ => Ok(Ctr::None),
    }
  }

  pub fn interpret_block(&mut self, block: Rc<Block>) -> Result<Ctr, Error> {
    for stmt in &block.stmts {
      let res = self.interpret_stmt(stmt);
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

  pub fn interpret(&mut self, comp_unit: &CompUnit) -> Result<RetVal, Error> {
    let mut block_: Option<Rc<Block>> = None;
    for stmt in &comp_unit.block.stmts {
      match stmt {
        Stmt::FuncDef( func_def ) => {
          let func_name = func_def.ident.clone();
          if func_name == "main" {
            block_ = Some(func_def.block.clone());
            break;
          }
        },
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

    if block_.is_none() {
      return Err(Error::RuntimeError("main function not found".to_string()));
    }
    let block = block_.clone().unwrap();

    let res = self.interpret_block(block);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    match res.clone().unwrap() {
      Ctr::Return( v ) => Ok(v),
      _ => Ok(RetVal::Void),
    }
  }
}