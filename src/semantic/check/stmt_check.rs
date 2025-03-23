use crate::ast::stmt::Stmt;
use crate::error::Error;

use super::Analyzer;

impl Analyzer {
  /// 以 Stmt 为单位进行语义检查
  pub fn stmt_check(&mut self, stmt: &Stmt) -> Result<(), Error> {
    match stmt {
      Stmt::Break => self.break_check(),
      Stmt::Continue => self.continue_check(),
      Stmt::ConstDecl( const_decl ) => self.const_decl_check(const_decl),
      Stmt::VarDecl( var_decl ) => self.var_decl_check(var_decl),
      Stmt::Asgn( asgn ) => self.asgn_check(&asgn),
      Stmt::Return( expr_ ) => self.return_check(&expr_),
      Stmt::Block( block ) => self.block_check(block.clone()),
      Stmt::While( while_ ) => self.while_check(while_),
      Stmt::FuncDef( func_def ) => self.func_def_check(func_def.clone()),
      Stmt::IfElse( ifelse ) => self.ifelse_check(ifelse),
      Stmt::Expr( expr_ ) => match expr_.is_some() {
        true => self.expr_check(expr_.as_ref().unwrap()),
        false => Ok(()),
      },
    }
  }
}