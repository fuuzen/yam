pub mod blocks;
pub mod scope;
pub mod symbol;
pub mod expr_check;

use std::collections::HashMap;
use std::rc::Rc;
use scope::BlockScope;

use crate::ast::block::{Block, BlockId};
use crate::ast::btype::LVal;
use crate::ast::expr::LOrExpr;
use crate::ast::func::{FuncCall, FuncDef, FuncType, FuncFParam};
use crate::ast::stmt::{Asgn, ConstDecl, ConstDef, Stmt, VarDecl, VarDef};
use crate::ast::comp_unit::CompUnit;
use crate::error::Error;

pub struct Analyzer {
  /// 作为全局作用域的 Global Block 的 Id
  global_block_id: BlockId,

  /// 指向当前分析检查到的 Block 的 Id
  current_block_id: BlockId,
  
  current_block: Option<Rc<Block>>,
  
  current_scope: Option<Rc<BlockScope>>,

  /// 当前分析检查到的 Block 处于几层循环中。
  /// 初始值为 0，进入一个循环 Block 时增加 1，离开时减 1。
  current_loop: i64,

  scope_table: HashMap<BlockId, Rc<BlockScope>>,
  block_table: HashMap<BlockId, Rc<Block>>,
}

impl Analyzer {
  pub fn new() -> Self {
    Self {
      global_block_id: 0,
      current_block_id: 0,
      current_block: None,
      current_scope: None,
      current_loop: 0,
      scope_table: HashMap::new(),
      block_table: HashMap::new(),
    }
  }

  /// 设置作为全局作用域的 Global Block 的 Id
  pub fn set_global_block(&mut self, block_id: BlockId) {
    self.global_block_id = block_id;
  }

  /// 获取作为全局作用域的 Global Block 的 Id
  pub fn get_global_block(&self) -> BlockId {
    self.global_block_id
  }

  /// 设置当前分析检查到的 Block 的 Id
  pub fn set_current_block(&mut self, block_id: BlockId) -> Result<(), Error> {
    self.current_block_id = block_id;
    let res = self.get_block_by_id(&block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    self.current_block = Some(res.unwrap().clone());

    let res = self.get_scope_by_id(&block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    self.current_scope = Some(res.unwrap().clone());
    Ok(())
  }

  /// 设置当前分析检查到的 Block 的 Id
  pub fn get_current_block_id(&self) -> BlockId {
    self.current_block_id
  }

  /// 设置当前分析检查到的 Block 的 Id
  pub fn get_current_block(&self) -> Rc<Block> {
    self.current_block.clone().unwrap()
  }

  /// 设置当前分析检查到的 Block 的 Id
  pub fn get_current_scope(&self) -> Rc<BlockScope> {
    self.current_scope.clone().unwrap()
  }

  /// 进入一个循环，需要 current_loop + 1
  fn enter_loop(&mut self) {
    self.current_loop += 1;
  }

  /// 离开一个循环，需要 current_loop - 1
  fn leave_loop(&mut self) {
    self.current_loop -= 1;
  }

  /// 常量声明的检查
  pub fn const_decl_check(&mut self, const_decl: &ConstDecl) -> Result<(), Error> {
    let len = const_decl.const_defs.len();
    for i in 0..len {
      let const_def = &const_decl.const_defs[i];
      let ConstDef{ident, expr} = const_def;

      let scope = self.get_current_scope();

      let rval = const_decl.rvals[i].clone();
      let mut res = scope.decl(&const_decl.btype, ident, true, rval);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      res = self.expr_check( expr);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }

  /// 变量声明的检查
  pub fn var_decl_check(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
    let len = var_decl.var_defs.len();
    for i in 0..len {
      let var_def = &var_decl.var_defs[i];
      let VarDef{ident, expr_} = var_def;

      let scope = self.get_current_scope();

      let rval = var_decl.rvals[i].clone();
      let mut res = scope.decl(&var_decl.btype, ident, false, rval.clone());
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      if expr_.is_some() {
        res = self.expr_check(expr_.as_ref().unwrap());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      }
    }
    Ok(())
  }
  
  /// 检查对一个变量 LVal 的赋值是否合法。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在且合法。
  /// 类型检查通过调用表达式检查实现，即检查表达式结果类型。
  pub fn asgn_check(&mut self, asgn: &Asgn) -> Result<(), Error> {
    let lval = &asgn.lval;
    let mut res = self.lval_check(lval);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let rval_ = lval.rval.borrow().clone();
    if rval_.is_none() {
      return Err(Error::InternalError(format!("{} was declared but RVal of {} was not bound", lval.ident, lval.ident)));
    }

    res = self.expr_check(&asgn.expr);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    Ok(())
  }

  /// 变量或常量调用的检查。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在。
  pub fn lval_check(&mut self, lval: &LVal) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;
    
    let mut scope = self.get_current_scope();

    let mut res_rval = scope.lval_check(lval);
    if res_rval.is_err() {
      return Err(res_rval.err().unwrap());
    }
    let mut rval_ = res_rval.unwrap();
    
    while rval_.is_none() {
      let block = self.get_current_block();

      let parent_id_ = block.get_parent_id();
      if parent_id_.is_none() {
        // 已经找遍所有父级 Block 了，该 LVal 不存在
        return Err(Error::SemanticError(format!("{} is not defined", lval.ident)));
      }
      let parent_id = parent_id_.unwrap();
      
      // 进入父级 Block
      let res = self.set_current_block(parent_id);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      scope = self.get_current_scope();

      res_rval = scope.lval_check(lval);
      if res_rval.is_err() {
        return Err(res_rval.err().unwrap());
      }
      rval_ = res_rval.unwrap();
    }

    // 恢复当前 Block Id
    let res = self.set_current_block(cur_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 绑定 RVal
    lval.bind_rval(rval_.unwrap());

    Ok(())
  }

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

  /// 以 Stmt 为单位进行语义检查
  pub fn stmt_check(&mut self, stmt: &Stmt) -> Result<(), Error> {
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
        let res = self.const_decl_check(const_decl);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::VarDecl( var_decl ) => {
        let res = self.var_decl_check(var_decl);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Asgn( asgn ) => {
        let res = self.asgn_check(&asgn);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Return( expr_ ) => {
        let res = self.return_check(&expr_);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::Block( block ) => {
        // 设置 Block 的 parent_id 为上一级 Block
        block.set_parent_id(cur_block_id);

        // 在 Blocks 表中添加这一 Block
        let mut res = self.add_block(block.clone());
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        // 在 Scopes 表中添加这一 Block
        res = self.add_scope(block.get_id());
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        // 进入 Block，必须先添加到 Blocks 和 Scopes 表再进入该 Block
        res = self.set_current_block(block.get_id());
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        // 进行 Block 为单位的语义检查
        res = self.block_check(block.clone());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        
        // 恢复当前 Block Id
        res = self.set_current_block(cur_block_id);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::While( while_ ) => {
        let mut res = self.expr_check(&while_.cond);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        self.enter_loop();

        res = self.stmt_check(&mut &while_.body);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        self.leave_loop();
      },
      Stmt::Expr( expr_ ) => {
        if expr_.is_some() {
          let res = self.expr_check(expr_.as_ref().unwrap());  
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        }
      },
      Stmt::FuncDef( func_def ) => {
        let res = self.func_def_check(func_def.clone());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      Stmt::IfElse( ifelse ) => {
        let mut res = self.expr_check(&ifelse.cond);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        res = self.stmt_check(&ifelse.if_);
        if res.is_err() {
          return Err(res.err().unwrap());
        }

        if ifelse.else_.is_some() {
          res = self.stmt_check(ifelse.else_.as_ref().unwrap());
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        }
      },
    }
    Ok(())
  }

  /// 以 Block 为单位对当前的 Block 进行语义检查。
  /// Blocks 和 Scopes 表中添加当前 Block、设置当前 Block 的 parent_id 的工作在调用该函数前完成。
  pub fn block_check(&mut self, block: Rc<Block>) -> Result<(), Error> {    
    let stmts = &block.stmts;
    for stmt in stmts {
      let res = self.stmt_check(&stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }

  /// 以 FuncDef 为单位进行语义检查。
  /// 认为函数的 Block 父级 Block 就是全局 Block，
  /// 将传入的参数视为声明的变量，然后进行 Block 为单位的语义检查。
  pub fn func_def_check(&mut self, func_def: Rc<FuncDef>) -> Result<(), Error> {
    // 获取当前作用域
    let mut scope = self.get_current_scope();

    // 检查当前作用域能否定义该函数
    let mut res = scope.func_def(func_def.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 函数定义 Block
    let block = func_def.block.clone();

    // 设置 Block 属于这个 FuncDef
    block.set_func(func_def.clone());

    // 设置函数的父级 Block 为全局 Block，使其能访问全局变量和全局常量
    block.set_parent_id(self.get_global_block());

    // 保存当前 Block Id，以便在函数定义 Block 的检查结束后恢复
    let cur_block_id = self.current_block_id;

    // 在 Blocks 表中添加当前 Block
    res = self.add_block(block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 在 Scopes 表中添加当前 Block
    res = self.add_scope(block.get_id());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 进入函数定义 Block，必须先添加到 Blocks 和 Scopes 表再进入该 Block
    res = self.set_current_block(block.get_id());
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    
    // 获取当前 Block 作用域
    scope = self.get_current_scope();

    // 函数参数视为声明的变量，进行声明检查
    for param in &func_def.func_fparams {
      let FuncFParam{ident, rval} = param;
      res = scope.decl(&param.get_btype(), ident, false, rval.clone());  /* 函数没有父级 Block，无需检查上层 */
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }

    // 进行 Block 为单位的语义检查
    res = self.block_check(block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    
    // 恢复当前 Block Id
    res = self.set_current_block(cur_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }

  /// 以 comp_unit 为单位进行语义检查。
  /// 检查无误后返回 Blocks 和 Scopes 供解释器读取。
  /// 全局变量常量的作用域视为一个 Block，这个 Block 没有父级 Block，不设置其 parent_id。
  pub fn check(&mut self, comp_unit: &CompUnit) -> Result<(), Error> {
    let block = comp_unit.block.clone();
    let block_id = block.get_id();

    self.set_global_block(block_id);
    
    // 在 Blocks 表中添加当前 Block
    let mut res = self.add_block(block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 在 Scopes 表中添加当前 Block
    res = self.add_scope(block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 必须先添加到 Blocks 和 Scopes 表再进入该 Block
    res = self.set_current_block(block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 进行 Block 为单位的语义检查
    let res = self.block_check(block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }
}