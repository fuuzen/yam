use crate::{ast::btype::LVal, error::Error};

use super::Analyzer;



impl Analyzer {

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
}