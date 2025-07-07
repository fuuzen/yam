use crate::ast::stmt::{ConstDecl, ConstDef, VarDecl, VarDef};
use crate::error::Error;

use super::Analyzer;

impl Analyzer {
  /// 常量声明的检查
  pub fn const_decl_check(&mut self, const_decl: &ConstDecl) -> Result<(), Error> {
    let len = const_decl.const_defs.len();
    for i in 0..len {
      let ConstDef{ident, rval: asgn_rval} = &const_decl.const_defs[i];

      self.get_current_scope().decl(
        ident,
        true,
        const_decl.rvals[i].clone()
      )?;

      self.asgn_rval_check(
        &asgn_rval,
        const_decl.btype
      )?;
    }
    Ok(())
  }

  /// 变量声明的检查
  pub fn var_decl_check(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
    let len = var_decl.var_defs.len();
    for i in 0..len {
      let VarDef{ident, rval_} =  &var_decl.var_defs[i];

      self.get_current_scope().decl(
        ident,
        false,
        var_decl.rvals[i].clone()
      )?;

      if rval_.is_some() {
        self.asgn_rval_check(
          rval_.as_ref().unwrap(),
          var_decl.btype
        )?;
      }
    }
    Ok(())
  }
}