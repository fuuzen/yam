use crate::ast::{func::FuncCall, val::LVal};

use super::expr::Expr;

/// 代表一个音符(可以是和弦)
#[derive(Debug)]
pub struct Note {
  pub notes: Vec<Expr>,

  // 表示音符的延长(倍数),在特定的 Measure 中才有意义
  pub len: Option<Expr>,
}

/// 表达式都被计算好后的 Note 值
#[derive(Debug, Clone)]
pub struct NoteValue {
  pub notes: Vec<i32>,
  pub len: Option<i32>,
}