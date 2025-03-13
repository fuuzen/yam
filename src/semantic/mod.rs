pub mod blocks;
pub mod scope;
pub mod symbol;
pub mod expr_check;

use std::rc::Rc;
use blocks::Blocks;
use expr_check::{bool_expr_check, expr_check, int_expr_check};
use scope::Scopes;

use crate::ast::block::BlockId;
use crate::ast::btype::{BType, LVal};
use crate::ast::expr::LOrExpr;
use crate::ast::func::{FuncCall, FuncDef, FuncType, FuncFParam};
use crate::ast::stmt::{Asgn, ConstDecl, ConstDef, Stmt, VarDecl, VarDef};
use crate::ast::track::{Def, Track};
use crate::error::Error;

pub struct Analyzer {
  /// 指向当前分析检查到的 Block 的 Id
  current_block_id: BlockId,

  /// 当前分析检查到的 Block 处于几层循环中。
  /// 初始值为 0，进入一个循环 Block 时增加 1，离开时减 1。
  current_loop: i64,
}

impl Analyzer {
  pub fn new() -> Self {
    Self {
      current_block_id: 0,
      current_loop: 0,
    }
  }

  /// 设置当前分析检查到的 Block 的 Id
  pub fn set_current_block(&mut self, block_id: BlockId) {
    self.current_block_id = block_id;
  }

  /// 进入一个循环，需要 current_loop + 1
  fn enter_loop(&mut self) {
    self.current_loop += 1;
  }

  /// 离开一个循环，需要 current_loop - 1
  fn leave_loop(&mut self) {
    self.current_loop -= 1;
  }

  /// 检查表达式是否合法及其返回类型是否匹配。
  pub fn expr_check_with_type(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, btype: &BType, lor_expr: &LOrExpr) -> Result<(), Error> {
    match btype {
      BType::Int => {
        int_expr_check(self, blocks, scopes, lor_expr)
      },
      BType::Bool => {
        bool_expr_check(self, blocks, scopes, lor_expr)
      }
    }
  }

  /// 检查表达式是否合法及其返回类型是否匹配。
  pub fn expr_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, lor_expr: &LOrExpr) -> Result<(), Error> {
    expr_check(self, blocks, scopes, lor_expr)
  }

  /// 常量声明的检查
  pub fn const_decl_check(&mut self, scopes: &mut Scopes, const_decl: &ConstDecl) -> Result<(), Error> {
    for const_def in &const_decl.const_defs {
      let ConstDef{ident, expr} = const_def;

      let res_scope = scopes.get_scope(&self.current_block_id);
      if res_scope.is_err() {
        return Err(res_scope.err().unwrap());
      }

      let res = res_scope.unwrap().decl(&const_decl.btype, ident, true, &self.current_block_id);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }

  /// 变量声明的检查
  pub fn var_decl_check(&mut self, scopes: &mut Scopes, var_decl: &VarDecl) -> Result<(), Error> {
    for var_def in &var_decl.var_defs {
      let VarDef{ident, expr_} = var_def;

      let res_scope = scopes.get_scope(&self.current_block_id);
      if res_scope.is_err() {
        return Err(res_scope.err().unwrap());
      }

      let res = res_scope.unwrap().decl(&var_decl.btype, ident, false, &self.current_block_id);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }
  
  /// 检查对一个变量 LVal 的赋值是否合法。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在且合法。
  /// 类型检查通过调用表达式检查实现，即检查表达式结果类型。
  pub fn asgn_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, asgn: &Asgn) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;

    let res_scope = scopes.get_scope(&cur_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }
    
    let res_bool = res_scope.unwrap().asgn_check(&asgn.lval);
    if res_bool.is_err() {
      return Err(res_bool.err().unwrap());
    }
    
    if !res_bool.unwrap() {
      let res_block = blocks.get_block(&cur_block_id);
      if res_block.is_err() {
        return Err(res_block.err().unwrap());
      }
      let parent_block_id = res_block.unwrap().parrent_id;
      
      if parent_block_id.is_none() {
        let ident = match asgn.lval {
          LVal::Ident(ref ident) => ident,
        };
        return Err(Error::SemanticError(format!("{} is not defined", ident)));
      }
      
      self.set_current_block(parent_block_id.unwrap());
      
      let res = self.asgn_check(blocks, scopes, asgn);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      self.set_current_block(cur_block_id);
      Ok(())
    } else {
      Ok(())
    }
  }

  /// 变量或常量调用的检查。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在。
  pub fn lval_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, lval: &LVal) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;
    
    let res_scope = scopes.get_scope(&cur_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }

    let res_bool = res_scope.unwrap().lval_check(lval);
    if res_bool.is_err() {
      return Err(res_bool.err().unwrap());
    }

    if !res_bool.unwrap() {
      let res_block = blocks.get_block(&cur_block_id);
      if res_block.is_err() {
        return Err(res_block.err().unwrap());
      }

      let parent_block_id = res_block.unwrap().parrent_id;
      
      if parent_block_id.is_none() {
        let ident = match lval {
          LVal::Ident(ident) => ident,
        };
        return Err(Error::SemanticError(format!("{} is not defined", ident)));
      }
      
      self.set_current_block(parent_block_id.unwrap());
      
      let res = self.lval_check(blocks, scopes, lval);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      self.set_current_block(cur_block_id);
      Ok(())
    } else {
      Ok(())
    }
  }

  /// 函数调用的检查，包括函数是否存在、参数是否符合函数定义。
  /// 由于目前 Base Type 只有 int(i32)，不需要检查对应参数是否类型匹配，
  /// 仅需检查参数数量是否匹配。
  pub fn func_call_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, func_call: &FuncCall) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;
    
    let res_scope = scopes.get_scope(&cur_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }

    let res_bool = res_scope.unwrap().func_call_check(func_call);
    if res_bool.is_err() {
      return Err(res_bool.err().unwrap());
    }
    
    if !res_bool.unwrap() {
      let cur_block_id = self.current_block_id;

      let res_block = blocks.get_block(&cur_block_id);
      if res_block.is_err() {
        return Err(res_block.err().unwrap());
      }

      let parent_block_id = res_block.unwrap().parrent_id;
      if parent_block_id.is_none() {
        return Err(Error::SemanticError(format!("{} is not defined", func_call.ident)));
      }
      
      self.set_current_block(parent_block_id.unwrap());
      
      let res = self.func_call_check(blocks, scopes, func_call);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      
      self.set_current_block(cur_block_id);
      Ok(())
    } else {
      Ok(())
    }
  }

  /// 检查 continue 是否有匹配的外层循环
  pub fn continue_check(&self) -> Result<(), Error> {
    match self.current_loop > 0 {
      true => Ok(()),
      false => Err(Error::SemanticError(format!("'continue' can't be used outside a loop"))),
    }
  }

  /// 检查 break 是否有匹配的外层循环
  pub fn break_check(&self) -> Result<(), Error> {
    match self.current_loop > 0 {
      true => Ok(()),
      false => Err(Error::SemanticError(format!("'break' can't be used outside a loop"))),
    }
  }

  /// return 类型是否符合函数定义的检查
  pub fn return_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, expr_: &Option<LOrExpr>) -> Result<(), Error> {
    let mut cur_block_id = self.current_block_id;

    let mut res_block = blocks.get_block(&cur_block_id).cloned();
    if res_block.is_err() {
      return Err(res_block.err().unwrap());
    }

    let mut parent_block_id = res_block.clone().unwrap().parrent_id;

    while parent_block_id.is_some() {
      cur_block_id = parent_block_id.unwrap();
  
      res_block = blocks.get_block(&cur_block_id).cloned();
      if res_block.is_err() {
        return Err(res_block.err().unwrap());
      }
  
      parent_block_id = res_block.clone().unwrap().parrent_id;
    }

    let block_ = res_block.clone().unwrap();
    let func_def_ = block_.func.as_ref();

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

        let res = self.expr_check_with_type(blocks, scopes, btype, expr_.as_ref().unwrap());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
    }

    Ok(())
  }

  /// 以 Stmt 为单位进行语义检查
  pub fn stmt_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, stmt: &Stmt) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;

    match stmt {
      Stmt::Break => {
        let res = self.break_check();
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Continue => {
        let res = self.continue_check();
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::ConstDecl( const_decl ) => {
        let res = self.const_decl_check(scopes, const_decl);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::VarDecl( var_decl ) => {
        let res = self.var_decl_check(scopes, var_decl);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Asgn( asgn ) => {
        let res = self.asgn_check(blocks, scopes, &asgn);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Return( expr_ ) => {
        let res = self.return_check(blocks, scopes, &expr_);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Block( block ) => {
        self.set_current_block(block.block_id);

        let mut res = blocks.add_block(block.clone());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
    
        res = scopes.add_scope(self.current_block_id);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        let res = self.block_check(blocks, scopes);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        
        self.set_current_block(cur_block_id);
      },
      Stmt::While( while_ ) => {
        let mut res = self.expr_check_with_type(blocks, scopes, &BType::Bool, &while_.cond);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        self.enter_loop();

        res = self.stmt_check(blocks, scopes, &while_.body);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        self.leave_loop();
      },
      Stmt::Expr( expr_ ) => {
        if expr_.is_some() {
          let res = self.expr_check_with_type(blocks, scopes, &BType::Int, expr_.as_ref().unwrap());  
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        }
      }
      _ => {}
    }

    let res_scope = scopes.get_scope(&cur_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }

    let res = res_scope.unwrap().if_else_check(stmt);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    Ok(())
  }

  /// 以 Block 为单位对当前的 Block 进行语义检查。
  /// 同时检查当前 Block 每一个 else 是否有匹配的 if。
  pub fn block_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes) -> Result<(), Error> {
    let res_block = blocks.get_block(&self.current_block_id);
    if res_block.is_err() {
      return Err(res_block.err().unwrap());
    }

    let stmts = &res_block.unwrap().clone().stmts;

    for stmt in stmts {
      let res = self.stmt_check(blocks, scopes, &stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }

  /// 以 func 为单位进行语义检查。
  /// 认为函数的 Block 是没有父级 Block 的，
  /// 将传入的参数视为声明的变量，然后进行 Block 为单位的语义检查。
  pub fn func_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, func_def: Rc<FuncDef>) -> Result<(), Error> {
    let mut res_scope = scopes.get_scope(&self.current_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }
    let mut scope = res_scope.unwrap();

    let mut res = scope.func_def(func_def.clone(), &self.current_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let cur_block_id = self.current_block_id;
    self.set_current_block(func_def.block.block_id);

    res = blocks.add_block(func_def.block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    res = scopes.add_scope(self.current_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    
    res_scope = scopes.get_scope(&self.current_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }
    scope = res_scope.unwrap();

    for param in &func_def.func_fparams {
      let FuncFParam{btype, ident} = param;
      res = scope.decl(btype, ident, false, &self.current_block_id);  /* 函数没有父级 Block，无需检查上层 */
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
  
    res = self.block_check(blocks, scopes);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    
    self.set_current_block(cur_block_id);
    Ok(())
  }

  /// 以 track 为单位进行语义检查
  /// 检查无误后返回 Blocks 和 Scopes 供解释器读取
  pub fn track_check(&mut self, track: &Track) -> Result<(Blocks, Scopes), Error> {
    self.set_current_block(track.block.block_id);

    let mut blocks = Blocks::new();
    let mut res = blocks.add_block(track.block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let mut scopes = Scopes::new();
    res = scopes.add_scope(self.current_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let res_scope = scopes.get_scope(&self.current_block_id);
    if res_scope.is_err() {
      return Err(res.err().unwrap());
    }
    let scope = res_scope.unwrap();

    let defs_ = &track.defs;
    if defs_.is_some() {
      for def in defs_.as_ref().unwrap() {
        match def {
          Def::ConstDecl( const_decl ) => {
            let ConstDecl{btype, const_defs} = const_decl;
            for const_def in const_defs {
              res = scope.decl(btype, &const_def.ident, true, &self.current_block_id);  /* track 没有父级 Block，无需检查上层 */
              if res.is_err() {
                return Err(res.err().unwrap());
              }
            }
          },
          Def::VarDecl( var_decl ) => {
            let VarDecl{btype, var_defs} = var_decl;
            for const_def in var_defs {
              res = scope.decl(btype, &const_def.ident, false, &self.current_block_id);  /* track 没有父级 Block，无需检查上层 */
              if res.is_err() {
                return Err(res.err().unwrap());
              }
            }
          },
          Def::FuncDef( func_def ) => {
            res = scope.func_def(func_def.clone(), &self.current_block_id);
            if res.is_err() {
              return Err(res.err().unwrap());
            }
          },
        }
      }
    }

    res = self.block_check(&mut blocks, &mut scopes);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok((blocks, scopes))
  }
}