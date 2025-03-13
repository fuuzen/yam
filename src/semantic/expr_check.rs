use crate::{ast::expr::{LOrExpr, PrimaryExpr, UnaryOp}, error::Error};

use super::{blocks::Blocks, scope::Scopes, Analyzer};

/// 检查表达式是否合法及其返回类型是否为 int 类型。
/// 由于目前 Base Type 只有 int(i32)，仅需要检查 int 和 bool 类型运算兼容性，
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
      for unary_op in &unary_expr.unary_ops {
        if *unary_op == UnaryOp::Not {
          return Err(Error::SemanticError(err));
        }
      }
      match &unary_expr.primary_exp {
        PrimaryExpr::LVal( lval ) => {
          let res = analyzer.lval_check(blocks, scopes, lval);
          if res.is_err() {
            return res;
          }
          /* 这里需要检查变量或常量类型不能是 bool，但目前没有 bool 类型，不做检查 */
        },
        PrimaryExpr::FuncCall( func_call ) => {
          let res = analyzer.func_call_check(blocks, scopes, func_call);
          if res.is_err() {
            return res;
          }
          /* 这里需要检查函数返回类型不能是 bool，但目前没有 bool 类型函数，不做检查 */
        },
        PrimaryExpr::Expr( expr ) => {
          /* 检查子表达式必须也是算术表达式 */
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
  let mut bool_op_count = lor_expr.land_exps.len() - 1;
  let mut calc_op_count = 0;
  for land_expr in &lor_expr.land_exps {
    bool_op_count += land_expr.eq_exps.len() - 1;
    for eq_expr in &land_expr.eq_exps {
      bool_op_count += eq_expr.eq_ops.len();
      for rel_expr in &eq_expr.rel_exps {
        bool_op_count += rel_expr.rel_ops.len();
        for add_expr in &rel_expr.add_exps {
          calc_op_count += add_expr.add_ops.len();
          for mul_expr in &add_expr.mul_exps {
            calc_op_count += mul_expr.mul_ops.len();
            for unary_expr in &mul_expr.unary_exps {
              for unary_op in &unary_expr.unary_ops {
                if *unary_op == UnaryOp::Not {
                  bool_op_count += 1;
                  if calc_op_count > 0 || unary_op != unary_expr.unary_ops.last().unwrap() {
                    /* 取非运算之上还有加减乘除 || 取非运算不是最后一个 UnaryOp */
                    return Err(Error::SemanticError(format!("alogorithm operation over logical negative '!' operation")));
                  }
                } else {
                  calc_op_count += 1;
                }
              }

              /*
               * 没有布尔运算，却有算数运算，表达式（若正确）将会是算术表达式，但结果可以直接转化为布尔，不算错误
               * if bool_op_count == 0 && calc_op_count > 0 {}
               */
              
              match &unary_expr.primary_exp {
                PrimaryExpr::LVal( lval ) => {
                  if bool_op_count > 0 {
                    let res = analyzer.lval_check(blocks, scopes, lval);
                    if res.is_err() {
                      return res;
                    }
                    /* 若有算术运算，这里需要检查变量或常量类型不能是 bool，但目前没有 bool 类型，不做检查 */
                  } else {
                    /* 没有布尔运算，表达式（若正确）将会是算术表达式，但结果可以直接转化为布尔，不算错误 */
                  }
                },
                PrimaryExpr::FuncCall( func_call ) => {
                  let res = analyzer.func_call_check(blocks, scopes, func_call);
                  if res.is_err() {
                    return res;
                  }
                  /* 若有算术运算，这里需要检查函数返回类型不能是 bool，但目前没有 bool 类型函数，不做检查 */
                  /* 若没有布尔运算，目前没有 bool 类型函数只有 int 类型，不做检查；表达式（若正确）将会是算术表达式，但结果可以直接转化为布尔，不算错误 */
                },
                PrimaryExpr::Expr( expr ) => {
                  if calc_op_count > 0 {
                    /* 有算术运算，运算单元不能是 bool 类型 */
                    let res = int_expr_check(analyzer, blocks, scopes, expr);
                    if res.is_err() {
                      return res;
                    }
                  } else {
                    /* 没有算术运算，运算单元也可以是算术表达式，结果可以直接转化为布尔，不做检查 */
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
/// 由于目前 Base Type 只有 int(i32)，仅需要检查 int 和 bool 类型运算兼容性。
pub fn expr_check(analyzer: &mut Analyzer, blocks: &mut Blocks, scopes: &mut Scopes, lor_expr: &LOrExpr) -> Result<(), Error> {
  let mut calc_op_count = 0;
  for land_expr in &lor_expr.land_exps {
    for eq_expr in &land_expr.eq_exps {
      for rel_expr in &eq_expr.rel_exps {
        for add_expr in &rel_expr.add_exps {
          calc_op_count += add_expr.add_ops.len();
          for mul_expr in &add_expr.mul_exps {
            calc_op_count += mul_expr.mul_ops.len();
            for unary_expr in &mul_expr.unary_exps {
              for unary_op in &unary_expr.unary_ops {
                if *unary_op == UnaryOp::Not {
                  if calc_op_count > 0 || unary_op != unary_expr.unary_ops.last().unwrap() {
                    /* 取非运算之上还有加减乘除 || 取非运算不是最后一个 UnaryOp */
                    return Err(Error::SemanticError(format!("mismatch ")));
                  }
                } else {
                  calc_op_count += 1;
                }
              }
              match &unary_expr.primary_exp {
                PrimaryExpr::LVal( lval ) => {
                  let res = analyzer.lval_check(blocks, scopes, lval);
                  if res.is_err() {
                    return res;
                  }
                  /* 若有算术运算，这里需要检查变量或常量类型不能是 bool，但目前没有 bool 类型，不做检查 */
                },
                PrimaryExpr::FuncCall( func_call ) => {
                  let res = analyzer.func_call_check(blocks, scopes, func_call);
                  if res.is_err() {
                    return res;
                  }
                  /* 若有算术运算，这里需要检查函数返回类型不能是 bool，但目前没有 bool 类型函数，不做检查 */
                },
                PrimaryExpr::Expr( expr ) => {
                  if calc_op_count > 0 {
                    /* 有算术运算，运算单元不能是 bool 类型 */
                    let res = int_expr_check(analyzer, blocks, scopes, expr);
                    if res.is_err() {
                      return res;
                    }
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