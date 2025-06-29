use crate::ast::stmt::While;
use crate::ast::val::BType;
use crate::error::Error;

use super::Analyzer;

impl Analyzer {
  /// 进入一个循环，需要 current_loop + 1
  fn enter_loop(&mut self) {
    self.current_loop += 1;
  }

  /// 离开一个循环，需要 current_loop - 1
  fn leave_loop(&mut self) {
    self.current_loop -= 1;
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

  /// 以 Stmt::While 为单位进行语法检查
  pub fn while_check(&mut self, while_: &While) -> Result<(), Error> {
    let mut res = self.expr_check(&while_.cond, BType::Bool);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    self.enter_loop();

    res = self.stmt_check(&mut &while_.body);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    self.leave_loop();
    Ok(())
  }
}