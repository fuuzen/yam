use crate::{ast::{stmt::IfElse, val::BType}, error::Error};

use super::Analyzer;

impl Analyzer {
  /// If ... [Else ...] 语句的检查
  pub fn ifelse_check(&mut self, ifelse: &IfElse) -> Result<(), Error> {
    let mut res = self.expr_check(&ifelse.cond, BType::Bool);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    res = self.stmt_check(&ifelse.if_);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    if ifelse.else_.is_some() {
      res = self.stmt_check(ifelse.else_.as_ref().unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }
}