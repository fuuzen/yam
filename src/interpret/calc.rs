use crate::{ast::{expr::{AddExpr, AddOp, EqExpr, Expr, LAndExpr, MulExpr, MulOp, PrimaryExpr, RelExpr, RelOp, UnaryExpr, UnaryOp}, val::Value}, error::Error};

use super::{ctr::RetVal, Interpreter};

const ZERO: RetVal = RetVal::Value(Value::Int(0));
const ONE: RetVal = RetVal::Value(Value::Int(1));

// 假定语义检查已经排除了所有潜在问题，这里直接运算。

impl Interpreter {
  /// 计算一元表达式的值
  pub fn calc_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Result<RetVal, Error> {
    let mut unit = match &unary_expr.primary_exp {
      PrimaryExpr::Expr(expr_) => {
        let res = self.calc_expr(expr_);
        if res.is_err() {
          return Err(res.err().unwrap());
        };
        res.unwrap()
      },
      PrimaryExpr::FuncCall(func_call) => {
        let res = self.call_func(func_call);
        if res.is_err() {
          return Err(res.err().unwrap());
        };
        res.unwrap()
      },
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
    let mut res = self.calc_unary_expr(&mul_expr.unary_exps[0]);
    if res.is_err() {
      return Err(res.err().unwrap());
    };
    let mut prod = res.unwrap();

    let len = mul_expr.unary_exps.len();
    for i in 1..len {
      res = self.calc_unary_expr(&mul_expr.unary_exps[i]);
      if res.is_err() {
        return Err(res.err().unwrap());
      };

      prod = match mul_expr.mul_ops[i - 1] {
        MulOp::Mul => prod * res.unwrap(),
        MulOp::Div => prod / res.unwrap(),
        MulOp::Mod => prod % res.unwrap(),
      };
    }
    Ok(prod)
  }

  /// 计算加法表达式的值
  pub fn calc_add_expr(&mut self, add_expr: &AddExpr) -> Result<RetVal, Error> {
    let mut res = self.calc_mul_expr(&add_expr.mul_exps[0]);
    if res.is_err() {
      return Err(res.err().unwrap());
    };
    let mut sum = res.unwrap();
    
    let len = add_expr.mul_exps.len();
    for i in 1..len {
      res = self.calc_mul_expr(&add_expr.mul_exps[i]);
      if res.is_err() {
        return Err(res.err().unwrap());
      };

      sum = match add_expr.add_ops[i - 1] {
        AddOp::Add => sum + res.unwrap(),
        AddOp::Sub => sum - res.unwrap(),
      };
    }
    Ok(sum)
  }

  /// 计算 RelExpr 的值，结果为 1 或 0
  pub fn calc_rel_expr(&mut self, rel_expr: &RelExpr) -> Result<RetVal, Error> {
    let mut res = self.calc_add_expr(&rel_expr.add_exps[0]);
    if res.is_err() {
      return Err(res.err().unwrap());
    };
    let mut left = res.unwrap();

    let len = rel_expr.add_exps.len();
    for i in 1..len {
      res = self.calc_add_expr(&rel_expr.add_exps[i]);
      if res.is_err() {
        return Err(res.err().unwrap());
      };

      let right = res.unwrap();
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
    let mut res = self.calc_rel_expr(&eq_expr.rel_exps[0]);
    if res.is_err() {
      return Err(res.err().unwrap());
    };
    let mut left = res.unwrap();

    let len = eq_expr.rel_exps.len();
    for i in 1..len {
      res = self.calc_rel_expr(&eq_expr.rel_exps[i]);
      if res.is_err() {
        return Err(res.err().unwrap());
      };

      let right = res.unwrap();
      left = match eq_expr.eq_ops[i - 1] {
        crate::ast::expr::EqOp::Eq => if left == right { ONE } else { ZERO },
        crate::ast::expr::EqOp::Ne => if left != right { ONE } else { ZERO },
      };
    }
    Ok(left)
  }

  /// 计算 LAndExpr 的值，结果为 1 或 0
  pub fn calc_land_expr(&mut self, land_expr: &LAndExpr) -> Result<RetVal, Error> {
    let mut res = self.calc_eq_expr(&land_expr.eq_exps[0]);
    if res.is_err() {
      return Err(res.err().unwrap());
    };
    let mut left = res.unwrap();

    let len = land_expr.eq_exps.len();
    for i in 1..len {
      res = self.calc_eq_expr(&land_expr.eq_exps[i]);
      if res.is_err() {
        return Err(res.err().unwrap());
      };

      let right = res.unwrap();
      left = if left != ZERO && right != ZERO { ONE } else { ZERO };
    }
    Ok(left)
  }


  /// 计算 Expr 表达式，也就是计算 LOrExpr 的值
  pub fn calc_expr(&mut self, expr: &Expr) -> Result<RetVal, Error> {
    let mut res = self.calc_land_expr(&expr.land_exps[0]);
    if res.is_err() {
      return Err(res.err().unwrap());
    };
    let mut left = res.unwrap();

    let len = expr.land_exps.len();
    for i in 1..len {
      res = self.calc_land_expr(&expr.land_exps[i]);
      if res.is_err() {
        return Err(res.err().unwrap());
      };

      let right = res.unwrap();
      left = if left != ZERO || right != ZERO { ONE } else { ZERO };
    }
    Ok(left)
  }
}