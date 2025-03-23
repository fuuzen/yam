use std::{collections::HashMap, rc::Rc};

use crate::{ast::block::{Block, BlockId}, error::Error};

use super::Analyzer;


impl Analyzer {
  /// 将 block 记录在其 id 为索引的哈希表中
  pub fn add_block(&mut self, block: Rc<Block>) -> Result<(), Error> {
    let k = block.get_id();
    let res = self.block_table.insert(k, block.clone());
    if res.is_some() {
      return Err(Error::InternalError(format!("block id conflict: {}", k)));
    }
    Ok(())
  }

  /// 获取给定 BlockId 的父级 Block
  pub fn get_parent_block(&self, block_id: &BlockId) -> Result<&Rc<Block>, Error> {
    let err: String = format!("can't find parent block for this block");
    self.block_table.get(block_id).ok_or(Error::InternalError(err))
  }

  /// 获取给定 BlockId 的 Rc<Block>
  pub fn get_block(&self, block_id: &BlockId) -> Result<&Rc<Block>, Error> {
    let err: String = format!("can't find current block");
    self.block_table.get(block_id).ok_or(Error::InternalError(err))
  }

  /// 获取整个 block_table，用于交给解释器执行
  pub fn get_block_table(&self) -> &HashMap<BlockId, Rc<Block>> {
    &self.block_table
  }
}