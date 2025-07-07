use crate::{ast::{stmt::IfElse, val::BType}, error::Error};

use super::Analyzer;

impl Analyzer {
  /// If ... [Else ...] 语句的检查
  pub fn ifelse_check(&mut self, ifelse: &IfElse) -> Result<(), Error> {
    self.expr_check(&ifelse.cond, Some(BType::Bool))?;

    self.stmt_check(&ifelse.if_)?;

    if ifelse.else_.is_some() {
      self.stmt_check(ifelse.else_.as_ref().unwrap())?;
    }
    Ok(())
  }
}