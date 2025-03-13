use crate::{ast::expr::{Expr, PrimaryExpr}, error::Error};

use super::Interpreter;


/// 表达式计算器
pub struct ExprCalculator {

}

impl ExprCalculator {
  pub fn new() -> Self {
    Self {}
  }

  /// 计算表达式结果为 int 类型。
  /// 假定语义检查已经排除了所有潜在问题，这里直接运算。
  pub fn calc_int(&self, expr: &Expr, interpreter: &Interpreter) -> Result<i32, Error> {
    let add_expr = &expr.land_exps[0].eq_exps[0].rel_exps[0].add_exps[0];
    for mul_expr in &add_expr.mul_exps {
      for unary_expr in &mul_expr.unary_exps {
        let unit: i32 = match &unary_expr.primary_exp {
          PrimaryExpr::Expr(expr_) => {
            let res = self.calc_int(expr_, interpreter);
            if res.is_err() {
              return Err(res.err().unwrap());
            };
            res.unwrap()
          },
          PrimaryExpr::FuncCall(func_call) => {
            let res = interpreter.call_int_func(func_call);
            if res.is_err() {
              return Err(res.err().unwrap());
            };
            res.unwrap()
          },
          PrimaryExpr::LVal(lval) => {
            
          },
          PrimaryExpr::Number(v) => {
            
          },
        }
      }
    }
    Ok(0)
  }

  /// 计算表达式结果为 bool 类型
  pub fn calc_bool(&self, expr: Expr) -> Result<bool, Error> {

    Ok(true)
  }
}