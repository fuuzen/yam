use crate::{ast::stmt::Asgn, error::Error};

use super::Analyzer;

impl Analyzer {
  /// 检查对一个变量 LVal 的赋值是否合法。
  /// 从这一级 Block 开始不断往上层 Block 检查符号是否存在且合法。
  /// 类型检查通过调用表达式检查实现，即检查表达式结果类型。
  pub fn asgn_check(&mut self, asgn: &Asgn) -> Result<(), Error> {
    let lval = &asgn.lval;
    self.lval_check(lval)?;
    match lval.rval.borrow().clone() {
      None => Err(Error::InternalError(format!(
        "{} was declared but RVal of {} was not bound", lval.ident, lval.ident
      ))),
      Some( rval) => self.asgn_rval_check(&asgn.rval, rval.get_btype())
    }
  }
}