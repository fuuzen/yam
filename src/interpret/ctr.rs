use core::fmt;
use std::ops;

use crate::ast::val::Value;

/// 实现控制流的返回类型
#[derive(Debug)]
pub enum Ctr {
  Return(RetVal),
  Continue,
  Break,
  None
}

/// 在 Value 的基础上增加一个形式上的 Void
#[derive(Debug, Clone)]
pub enum RetVal {
  Value(Value),
  Void,
}

impl fmt::Display for RetVal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RetVal::Void => write!(f, "void"),
      RetVal::Value(v) => write!(f, "{v}"),
    }
  }
}

/// 一元运算符 !
impl ops::Not for RetVal {
  type Output = RetVal;

  fn not(self) -> Self::Output {
    match self {
      RetVal::Value(Value::Int(i)) => RetVal::Value(Value::Int(!i)),
      _ => self,
    }
  }
}

/// 一元运算符 -
impl ops::Neg for RetVal {
  type Output = RetVal;

  fn neg(self) -> Self::Output {
    match self {
      RetVal::Value(Value::Int(i)) => RetVal::Value(Value::Int(-i)),
      _ => self,
    }
  }
}

/// 二元运算符 + - * / %
macro_rules! impl_bin_op {
  ($trait:ident, $op:ident, $method:ident) => {
    impl ops::$trait for RetVal {
      type Output = RetVal;

      fn $method(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
          (RetVal::Value(Value::Int(a)), RetVal::Value(Value::Int(b))) => {
            RetVal::Value(Value::Int(a.$method(b)))
          }
          (a, _) => a,
        }
      }
    }
  };
}

impl_bin_op!(Add, add, add);
impl_bin_op!(Sub, sub, sub);
impl_bin_op!(Mul, mul, mul);
impl_bin_op!(Div, div, div);
impl_bin_op!(Rem, rem, rem);

/// 二元逻辑运算符 >= <=
impl PartialOrd for RetVal {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (RetVal::Value(Value::Int(a)), RetVal::Value(Value::Int(b))) => a.partial_cmp(b),
      _ => None,
    }
  }

  fn le(&self, other: &Self) -> bool {
    match (self, other) {
      (RetVal::Value(Value::Int(a)), RetVal::Value(Value::Int(b))) => a <= b,
      _ => false,
    }
  }

  fn ge(&self, other: &Self) -> bool {
    match (self, other) {
      (RetVal::Value(Value::Int(a)), RetVal::Value(Value::Int(b))) => a >= b,
      _ => false,
    }
  }
}

/// 二元比较运算符 == !=
impl PartialEq for RetVal {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (RetVal::Value(a), RetVal::Value(b)) => a == b,
      (RetVal::Void, RetVal::Void) => true,
      _ => false,
    }
  }
}

impl Eq for RetVal {}

impl ops::Not for &RetVal {
  type Output = RetVal;

  fn not(self) -> Self::Output {
    match self {
      RetVal::Value(Value::Int(i)) => RetVal::Value(Value::Int(!i)),
      _ => self.clone(),
    }
  }
}

impl ops::Neg for &RetVal {
  type Output = RetVal;

  fn neg(self) -> Self::Output {
    match self {
      RetVal::Value(Value::Int(i)) => RetVal::Value(Value::Int(-i)),
      _ => self.clone(),
    }
  }
}

/// 为引用类型实现二元运算符
macro_rules! impl_bin_op_ref {
  ($trait:ident, $op:ident, $method:ident) => {
    impl ops::$trait for &RetVal {
      type Output = RetVal;

      fn $method(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
          (RetVal::Value(Value::Int(a)), RetVal::Value(Value::Int(b))) => {
            RetVal::Value(Value::Int(a.$method(b)))
          }
          (a, _) => a.clone(),
        }
      }
    }
  };
}

impl_bin_op_ref!(Add, add, add);
impl_bin_op_ref!(Sub, sub, sub);
impl_bin_op_ref!(Mul, mul, mul);
impl_bin_op_ref!(Div, div, div);
impl_bin_op_ref!(Rem, rem, rem);