use std::rc::Rc;

use crate::ast::block::Block;
use crate::ast::func::{FuncDef, FuncFParam};
use crate::ast::score::Score;
use crate::error::Error;

use super::Analyzer;

impl Analyzer {
  /// 以普通 Block 为单位对当前的 Block 进行语义检查。
  /// - Blocks 和 Scopes 表中添加当前 Block;
  /// - 设置当前 Block 的 parent_id 为 Analyzer 的 current_block_id;
  /// - 设置 Analyzer 的 current_block 为当前 Block;
  /// - 遍历并检查所有 stmt;
  /// - 恢复 Analyzer 的 current_block.
  pub fn block_check(&mut self, block: Rc<Block>) -> Result<(), Error> {
    let cur_block_id = self.get_current_block_id();

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

    // 遍历并检查所有 stmt
    let stmts = &block.stmts;
    for stmt in stmts {
      let res = self.stmt_check(&stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }

    // 恢复当前 Block Id
    res = self.set_current_block(cur_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }

  /// 以 global Block 为单位对当前的 Block 进行语义检查。
  /// 全局变量、常量、函数的作用域视为一个 Block，这个特殊 Block 就是 global Block。
  /// 这个 Block 没有父级 Block，不设置其 parent_id，其 parent_id 将保持为 None。
  /// - 设置全局 Block 为 CompUnit 的 Block;
  /// - Blocks 和 Scopes 表中添加当前 Block;
  /// - 设置 Analyzer 的 current_block 为当前 Block;
  /// - 遍历并检查所有 stmt;
  pub fn global_block_check(&mut self, block: Rc<Block>) -> Result<(), Error> {
    let block_id = block.get_id();

    // 设置全局 Block 为 CompUnit 的 Block
    self.set_global_block(block_id);

    // 在 Blocks 表中添加这一 Block
    let mut res = self.add_block(block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 在 Scopes 表中添加这一 Block
    res = self.add_scope(block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 进入 Block，必须先添加到 Blocks 和 Scopes 表再进入该 Block
    res = self.set_current_block(block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 遍历并检查所有 stmt
    let stmts = &block.stmts;
    for stmt in stmts {
      let res = self.stmt_check(&stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }

    Ok(())
  }
  
  /// 以属于函数的 Block 为单位对当前的 Block 进行语义检查。
  /// 认为函数的 Block 父级 Block 就是全局 Block，
  /// 将传入的参数视为声明的变量，然后进行 Block 为单位的语义检查。
  /// - Blocks 和 Scopes 表中添加当前 Block;
  /// - 设置当前 Block 的 parent_id 为 global_block_id;
  /// - 设置 Analyzer 的 current_block 为当前 Block;
  /// - 对所有参数进行声明检查;
  /// - 遍历并检查所有 stmt;
  /// - 恢复 Analyzer 的 current_block.
  pub fn func_block_check(&mut self, func_def: Rc<FuncDef>) -> Result<(), Error> {
    // 函数定义 Block
    let block = func_def.block.clone();

    // 设置 Block 属于这个 FuncDef
    block.set_func(func_def.clone());

    // 设置函数的父级 Block 为全局 Block，使其能访问全局变量和全局常量
    block.set_parent_id(self.get_global_block());

    // 保存当前 Block Id，以便在函数定义 Block 的检查结束后恢复
    let cur_block_id = self.current_block_id;

    // 在 Blocks 表中添加当前 Block
    let mut res = self.add_block(block.clone());
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
    let scope = self.get_current_scope();

    // 函数参数视为声明的变量，进行声明检查
    for param in &func_def.func_fparams {
      let FuncFParam{ident, rval} = param;
      res = scope.decl(ident, false, rval.clone());  /* 函数没有父级 Block，无需检查上层 */
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }

    // 遍历并检查所有 stmt
    let stmts = &block.stmts;
    for stmt in stmts {
      let res = self.stmt_check(&stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    
    // 恢复当前 Block Id
    res = self.set_current_block(cur_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }
  
  /// 要做的事情和 func_block 差不多,只是多了 channel_stmt 的 check
  pub fn score_check(&mut self, score: &Score) -> Result<(), Error> {
    // 函数定义 Block
    let block = score.block.clone();

    // 设置函数的父级 Block 为全局 Block，使其能访问全局变量和全局常量
    block.set_parent_id(self.get_global_block());

    // 保存当前 Block Id，以便在函数定义 Block 的检查结束后恢复
    let cur_block_id = self.current_block_id;

    // 在 Blocks 表中添加当前 Block
    let mut res = self.add_block(block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 在 Scopes 表中添加当前 Block
    res = self.add_scope(block.get_id());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 进入 Score Block，必须先添加到 Blocks 和 Scopes 表再进入该 Block
    res = self.set_current_block(block.get_id());
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    // 遍历并检查所有 stmt
    let stmts = &block.stmts;
    for stmt in stmts {
      let res = self.stmt_check(&stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }

    // 遍历检查所有 channel stmt
    let stmts = &score.channel_stmts;
    for stmt in stmts {
      let res = self.channel_stmt_check(&stmt);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    
    // 恢复当前 Block Id
    res = self.set_current_block(cur_block_id);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }
}