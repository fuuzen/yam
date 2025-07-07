use crate::{ast::val::LVal, error::Error};

use super::Analyzer;


impl Analyzer {
  /// 变量或常量调用的检查。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在。
  pub fn lval_check(&mut self, lval: &LVal) -> Result<(), Error> {
    let cur_block_id = self.current_block_id;
    
    let mut scope = self.get_current_scope();

    let mut rval_ = scope.lval_check(lval)?;
    
    while rval_.is_none() {
      let block = self.get_current_block();

      let parent_id_ = block.get_parent_id();
      if parent_id_.is_none() {
        // 已经找遍所有父级 Block 了，该 LVal 不存在
        return Err(Error::SemanticError(format!("{} is not defined", lval.ident)));
      }
      let parent_id = parent_id_.unwrap();
      
      // 进入父级 Block
      self.set_current_block(parent_id)?;

      scope = self.get_current_scope();

      rval_ = scope.lval_check(lval)?;
    }

    // 恢复当前 Block Id
    self.set_current_block(cur_block_id)?;

    // 绑定 RVal
    lval.bind_rval(rval_.unwrap());

    Ok(())
  }
}