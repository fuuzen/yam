use crate::{ast::stmt::{Asgn, AsgnRVal}, error::Error};

use super::Analyzer;

impl Analyzer {
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

    match &asgn.rval {
      AsgnRVal::Expr( expr ) => {
        res = self.expr_check(expr);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      _ => unimplemented!()  // TODO
    }

    Ok(())
  }
}