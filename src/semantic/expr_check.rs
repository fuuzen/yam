use crate::{ast::expr::{LOrExpr, PrimaryExpr}, error::Error};

use super::{blocks::Blocks, scope::Scopes, Analyzer};

/// 检查表达式是否合法及其返回类型是否为 int 类型。
/// 由于目前 Base Type 只有 int(i32)，不需要检查类型运算兼容性，
/// 仅需要表达式中出现的检查函数调用返回值非 Void。
pub fn int_expr_check(analyzer: &mut Analyzer, blocks: &mut Blocks, scopes: &mut Scopes, lor_expr: &LOrExpr) -> Result<(), Error> {
  let err = format!("bool expression assigning to int value");
  if lor_expr.land_exps.len() != 1 {
    return Err(Error::SemanticError(err));
  }
  let land_expr = &lor_expr.land_exps[0];
  if land_expr.eq_exps.len() != 1 {
    return Err(Error::SemanticError(err));
  }
  let eq_expr = &land_expr.eq_exps[0];
  if eq_expr.rel_exps.len() != 1 {
    return Err(Error::SemanticError(err));
  }
  let rel_expr = &eq_expr.rel_exps[0];
  if rel_expr.add_exps.len() != 1 {
    return Err(Error::SemanticError(err));
  }
  let add_expr = &rel_expr.add_exps[0];
  for mul_expr in &add_expr.mul_exps {
    for unary_expr in &mul_expr.unary_exps {
      match &unary_expr.primary_exp {
        PrimaryExpr::LVal( lval ) => {
          let res = analyzer.lval_check(blocks, scopes, lval);
          if res.is_err() {
            return res;
          }
        },
        PrimaryExpr::FuncCall( func_call ) => {
          let res = analyzer.func_call_check(blocks, scopes, func_call);
          if res.is_err() {
            return res;
          }
        },
        PrimaryExpr::Expr( expr ) => {
          let res = int_expr_check(analyzer, blocks, scopes, expr);
          if res.is_err() {
            return res;
          }
        },
        PrimaryExpr::Number( _ ) => {}
      }
    }
  }
  Ok(())
}

/// 检查表达式是否合法及其返回类型是否为 bool 类型。
/// 由于目前 Base Type 只有 int(i32)，bool 类型实际上并不存在而是用 int 代替，
/// 相当于检查返回类型是否为 int 类型，但为了区分和便于扩展还是单独定义一个函数。
pub fn bool_expr_check(analyzer: &mut Analyzer, blocks: &mut Blocks, scopes: &mut Scopes, lor_expr: &LOrExpr) -> Result<(), Error> {
  let err = format!("int expression assigning to bool value");
  let mut bool_op_count = lor_expr.land_exps.len() - 1;
  for land_expr in &lor_expr.land_exps {
    bool_op_count += land_expr.eq_exps.len() - 1;
    for eq_expr in &land_expr.eq_exps {
      bool_op_count += eq_expr.eq_ops.len();
      for rel_expr in &eq_expr.rel_exps {
        bool_op_count += rel_expr.rel_ops.len();
        if bool_op_count == 0 {
          return Err(Error::SemanticError(err));
        }
        for add_expr in &rel_expr.add_exps {
          for mul_expr in &add_expr.mul_exps {
            for unary_expr in &mul_expr.unary_exps {
              match &unary_expr.primary_exp {
                PrimaryExpr::LVal( lval ) => {
                  let res = analyzer.lval_check(blocks, scopes, lval);
                  if res.is_err() {
                    return res;
                  }
                },
                PrimaryExpr::FuncCall( func_call ) => {
                  let res = analyzer.func_call_check(blocks, scopes, func_call);
                  if res.is_err() {
                    return res;
                  }
                },
                PrimaryExpr::Expr( expr ) => {
                  let res = int_expr_check(analyzer, blocks, scopes, expr);
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

/// 检查表达式是否合法，对返回类型不做检查。
/// 由于目前 Base Type 只有 int(i32)，不需要检查类型运算兼容性。
pub fn expr_check(analyzer: &mut Analyzer, blocks: &mut Blocks, scopes: &mut Scopes, lor_expr: &LOrExpr) -> Result<(), Error> {
  for land_expr in &lor_expr.land_exps {
    for eq_expr in &land_expr.eq_exps {
      for rel_expr in &eq_expr.rel_exps {
        for add_expr in &rel_expr.add_exps {
          for mul_expr in &add_expr.mul_exps {
            for unary_expr in &mul_expr.unary_exps {
              match &unary_expr.primary_exp {
                PrimaryExpr::LVal( lval ) => {
                  let res = analyzer.lval_check(blocks, scopes, lval);
                  if res.is_err() {
                    return res;
                  }
                },
                PrimaryExpr::FuncCall( func_call ) => {
                  let res = analyzer.func_call_check(blocks, scopes, func_call);
                  if res.is_err() {
                    return res;
                  }
                },
                PrimaryExpr::Expr( expr ) => {
                  let res = int_expr_check(analyzer, blocks, scopes, expr);
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