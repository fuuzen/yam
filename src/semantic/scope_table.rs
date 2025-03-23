use std::rc::Rc;

use crate::ast::block::BlockId;
use crate::error::Error;

use super::block_scope::BlockScope;
use super::Analyzer;

impl Analyzer {
  /// 在哈希表中创建一个给定 BlockId 的新的 BlockScope
  pub fn add_scope(&mut self, block_id: BlockId) -> Result<(), Error> {
    let res = self.scope_table.insert(block_id, Rc::new(BlockScope::new()));
    if res.is_some() {
      return Err(Error::InternalError(format!("block id conflict: {}", block_id)));
    }
    Ok(())
  }

  /// 获取给定 BlockId 的父级 scope
  pub fn get_parent_scope(&self, block_id: &BlockId) -> Result<Rc<BlockScope>, Error> {
    self.scope_table.get(block_id).ok_or(Error::InternalError(format!("can't find parent block for this block {}", block_id))).cloned()
  }

  /// 获取给定 BlockId 的 &mut BlockScope
  pub fn get_scope_by_id(&mut self, block_id: &BlockId) -> Result<Rc<BlockScope>, Error> {
    self.scope_table.get_mut(block_id).ok_or(Error::InternalError(format!("can't find the scope of this block: {}", block_id))).cloned()
  }
}