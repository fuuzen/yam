use crate::ast::stmt::{AsgnRVal, ConstDecl, ConstDef, VarDecl, VarDef};
use crate::error::Error;

use super::Analyzer;

impl Analyzer {
  /// 常量声明的检查
  pub fn const_decl_check(&mut self, const_decl: &ConstDecl) -> Result<(), Error> {
    let len = const_decl.const_defs.len();
    for i in 0..len {
      let const_def = &const_decl.const_defs[i];
      let ConstDef{ident, rval: asgn_rval} = const_def;

      let scope = self.get_current_scope();

      let rval = const_decl.rvals[i].clone();
      let mut res = scope.decl(ident, true, rval);
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      match &asgn_rval {
        AsgnRVal::Expr( expr ) => {
          res = self.expr_check(expr);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        },
        _ => unimplemented!()  // TODO
      }
    }
    Ok(())
  }

  /// 变量声明的检查
  pub fn var_decl_check(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
    let len = var_decl.var_defs.len();
    for i in 0..len {
      let var_def = &var_decl.var_defs[i];
      let VarDef{ident, rval_: asgn_rval_} = var_def;

      let scope = self.get_current_scope();

      let rval = var_decl.rvals[i].clone();
      let mut res = scope.decl(ident, false, rval.clone());
      if res.is_err() {
        return Err(res.err().unwrap());
      }

      if asgn_rval_.is_some() {
        match asgn_rval_.as_ref().unwrap() {
          AsgnRVal::Expr( expr ) => {
            res = self.expr_check(expr);
            if res.is_err() {
              return Err(res.err().unwrap());
            }
          },
          _ => unimplemented!()  // TODO
        }
      }
    }
    Ok(())
  }
}