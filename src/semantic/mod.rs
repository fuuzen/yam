pub mod scope;
pub mod symbol;
pub mod expr_check;

use std::collections::HashMap;
use std::rc::Rc;
use expr_check::{bool_expr_check, int_expr_check};
use scope::BlockScope;

use crate::ast::block::{Block, BlockId};
use crate::ast::btype::BType;
use crate::ast::expr::LOrExpr;
use crate::ast::func::FuncCall;
use crate::ast::stmt::{Asgn, Else, Stmt};
use crate::ast::track::Track;
use crate::error::Error;

pub struct Analyzer {
  /// BlockId 到具体 Block 到哈希表，用于快速定位回指定 Block。
  /// 这里的 Rc<Block> 指向的 Block 在 parse 阶段创建。
  block_table: HashMap<BlockId, Rc<Block>>,

  /// 所有的 scope，用哈希表存储
  scope_table: HashMap<BlockId, BlockScope>,
}

impl Analyzer {
  pub fn new() -> Self {
    Self {
      block_table: HashMap::new(),
      scope_table: HashMap::new(),
    }
  }

  /// 在哈希表中创建一个给定 block 的 scope，并将 block 记录在其 id 为索引的哈希表中
  pub fn create_scope(&mut self, block: Rc<Block>) -> Result<(), Error> {
    let mut err: String = format!("can't create scope for this block");
    let k = block.block_id;
    let res = self.scope_table.insert(k, BlockScope::new()).ok_or(Error::InternalError(err)).map(|_| ());
    if res.is_err() {
      return res;
    }
    err = format!("can't register this block to index-block table"); 
    self.block_table.insert(k, block).ok_or(Error::InternalError(err)).map(|_| ())
  }

  /// 获取给定 BlockId 的父级 Block
  pub fn get_parent_block(&self, block_id: &BlockId) -> Result<&Rc<Block>, Error> {
    let err: String = format!("can't find parent block for this block");
    self.block_table.get(block_id).ok_or(Error::InternalError(err))
  }

  /// 获取给定 BlockId 的父级 scope
  pub fn get_parent_scope(&self, block_id: &BlockId) -> Result<&BlockScope, Error> {
    let err: String = format!("can't find parent block for this block");
    self.scope_table.get(block_id).ok_or(Error::InternalError(err))
  }

  /// 检查表达式是否合法及其返回类型是否匹配。
  pub fn expr_check(&self, btype: BType, lor_expr: &LOrExpr) -> Result<(), Error> {
    match btype {
      BType::Int => {
        int_expr_check(self, lor_expr)
      },
      BType::Bool => {
        bool_expr_check(self, lor_expr)
      }
    }
  }
  
  /// 检查对一个变量 LVal 的赋值是否合法。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在且合法。
  /// 类型检查通过调用表达式检查实现，即检查表达式结果类型。
  pub fn asgn_check(&self, asgn: &Asgn) -> Result<(), Error> {
    Ok(())
  }

  /// 函数调用的检查，包括函数是否存在、参数是否符合函数定义。
  /// 由于目前 Base Type 只有 int(i32)，不需要检查对应参数是否类型匹配，
  /// 仅需检查参数数量是否匹配。
  pub fn func_call_check(&self, func_call: &FuncCall) -> Result<(), Error> {
    Ok(())
  }

  /// 检查 else 是否有匹配的 if
  pub fn else_check(&self, else_: &Else) -> Result<(), Error> {
    Ok(())
  }

  /// 给定 continue 或 break 所在父级 Block，检查其是否有匹配的 while
  pub fn continue_check(&self, block: Rc<Block>) -> Result<(), Error> {
    Ok(())
  }

  /// return 类型是否符合函数定义的检查
  pub fn return_check(&self, block: Rc<Block>, return_: &Stmt) -> Result<(), Error> {
    Ok(())
  }

  /// 分析 AST 并进行语义检查
  pub fn analyze(&self, track: &Track) -> Result<(), Error> {
    Ok(())
  }
}