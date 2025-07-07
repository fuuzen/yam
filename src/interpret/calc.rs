use crate::{ast::{expr::{AddExpr, AddOp, EqExpr, Expr, LAndExpr, MulExpr, MulOp, PrimaryExpr, RelExpr, RelOp, UnaryExpr, UnaryOp}, val::Value}, error::Error};

use super::{ctr::RetVal, Interpreter};

const ZERO: RetVal = RetVal::Value(Value::Int(0));
const ONE: RetVal = RetVal::Value(Value::Int(1));

// 假定语义检查已经排除了所有潜在问题，这里直接运算。

impl Interpreter {
  /// 计算一元表达式的值
  pub fn calc_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Result<RetVal, Error> {
    let mut unit = match &unary_expr.primary_exp {
      PrimaryExpr::Expr(expr_) => self.calc_expr(expr_)?,
      PrimaryExpr::FuncCall(func_call) => self.call_func(func_call)?,
      PrimaryExpr::LVal(lval) => RetVal::Value(lval.get_value()),
      PrimaryExpr::Number(v) => RetVal::Value(Value::Int(v.clone())),
    };
    for op in &unary_expr.unary_ops { // TODO:暂时没规定和检查顺序
      match op {
        UnaryOp::Minus => unit = -unit,
        UnaryOp::Not => unit = if unit == ZERO { ONE } else { ZERO },
        _ => (),
      }
    }
    Ok(unit)
  }

  /// 计算乘法表达式的值
  pub fn calc_mul_expr(&mut self, mul_expr: &MulExpr) -> Result<RetVal, Error> {
    let mut prod = self.calc_unary_expr(&mul_expr.unary_exps[0])?;
    let len = mul_expr.unary_exps.len();
    for i in 1..len {
      let right = self.calc_unary_expr(&mul_expr.unary_exps[i])?;
      prod = match mul_expr.mul_ops[i - 1] {
        MulOp::Mul => prod * right,
        MulOp::Div => prod / right,
        MulOp::Mod => prod % right,
      };
    }
    Ok(prod)
  }

  /// 计算加法表达式的值
  pub fn calc_add_expr(&mut self, add_expr: &AddExpr) -> Result<RetVal, Error> {
    let mut sum = self.calc_mul_expr(&add_expr.mul_exps[0])?;
    let len = add_expr.mul_exps.len();
    for i in 1..len {
      let right = self.calc_mul_expr(&add_expr.mul_exps[i])?;
      sum = match add_expr.add_ops[i - 1] {
        AddOp::Add => sum + right,
        AddOp::Sub => sum - right,
      };
    }
    Ok(sum)
  }

  /// 计算 RelExpr 的值，结果为 1 或 0
  pub fn calc_rel_expr(&mut self, rel_expr: &RelExpr) -> Result<RetVal, Error> {
    let mut left = self.calc_add_expr(&rel_expr.add_exps[0])?;
    let len = rel_expr.add_exps.len();
    for i in 1..len {
      let right = self.calc_add_expr(&rel_expr.add_exps[i])?;
      left = match rel_expr.rel_ops[i - 1] {
        RelOp::Lt => if left < right { ONE } else { ZERO },
        RelOp::Le => if left <= right { ONE } else { ZERO },
        RelOp::Gt => if left > right { ONE } else { ZERO },
        RelOp::Ge => if left >= right { ONE } else { ZERO },
      };
    }
    Ok(left)
  }

  /// 计算 EqExpr 的值
  pub fn calc_eq_expr(&mut self, eq_expr: &EqExpr) -> Result<RetVal, Error> {
    let mut left = self.calc_rel_expr(&eq_expr.rel_exps[0])?;
    let len = eq_expr.rel_exps.len();
    for i in 1..len {
      let right = self.calc_rel_expr(&eq_expr.rel_exps[i])?;
      left = match eq_expr.eq_ops[i - 1] {
        crate::ast::expr::EqOp::Eq => if left == right { ONE } else { ZERO },
        crate::ast::expr::EqOp::Ne => if left != right { ONE } else { ZERO },
      };
    }
    Ok(left)
  }

  /// 计算 LAndExpr 的值，结果为 1 或 0
  pub fn calc_land_expr(&mut self, land_expr: &LAndExpr) -> Result<RetVal, Error> {
    let mut left = self.calc_eq_expr(&land_expr.eq_exps[0])?;
    let len = land_expr.eq_exps.len();
    for i in 1..len {
      let right =  self.calc_eq_expr(&land_expr.eq_exps[i])?;
      left = if left != ZERO && right != ZERO { ONE } else { ZERO };
    }
    Ok(left)
  }


  /// 计算 Expr 表达式，也就是计算 LOrExpr 的值
  pub fn calc_expr(&mut self, expr: &Expr) -> Result<RetVal, Error> {
    let mut left = self.calc_land_expr(&expr.land_exps[0])?;
    let len = expr.land_exps.len();
    for i in 1..len {
      let right = self.calc_land_expr(&expr.land_exps[i])?;
      left = if left != ZERO || right != ZERO { ONE } else { ZERO };
    }
    Ok(left)
  }
}