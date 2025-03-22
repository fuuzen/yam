use crate::{ast::expr::{LOrExpr, PrimaryExpr}, error::Error};

use super::{blocks::Blocks, scope::Scopes, Analyzer};


impl Analyzer {
  /// 检查表达式是否合法。
  /// 由于目前 Base Type 只有 int(i32)，int 和 bool 类型运算按照 C 语言标准兼容。
  /// 不需要额外的兼容检查，对返回类型也不做检查。
  pub fn expr_check(&mut self, blocks: &mut Blocks, scopes: &mut Scopes, lor_expr: &LOrExpr) -> Result<(), Error> {
    for land_expr in &lor_expr.land_exps {
      for eq_expr in &land_expr.eq_exps {
        for rel_expr in &eq_expr.rel_exps {
          for add_expr in &rel_expr.add_exps {
            for mul_expr in &add_expr.mul_exps {
              for unary_expr in &mul_expr.unary_exps {
                match &unary_expr.primary_exp {
                  PrimaryExpr::LVal( lval ) => {
                    let res = self.lval_check(blocks, scopes, lval);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::FuncCall( func_call ) => {
                    let res = self.func_call_check(blocks, scopes, func_call);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::Expr( expr ) => {
                    let res = self.expr_check(blocks, scopes, expr);
                    if res.is_err() {
                      return res;
                    }
                  },
                  PrimaryExpr::Number( _ ) => {}
                }
              }
            }
          }
        }
      }
    }
    Ok(())
  }
}