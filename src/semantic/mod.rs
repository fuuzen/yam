pub mod block_table;
pub mod block_scope;
pub mod scope_table;
pub mod symbol;
pub mod check;

use std::collections::HashMap;
use std::rc::Rc;
use block_scope::BlockScope;

use crate::ast::block::{Block, BlockId};
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
}