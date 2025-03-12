use crate::{ast::expr::LOrExpr, error::Error};

use super::Analyzer;

/// 检查表达式是否合法及其返回类型是否为 int 类型。
/// 由于目前 Base Type 只有 int(i32)，不需要检查类型运算兼容性，
/// 仅需要表达式中出现的检查函数调用返回值非 Void。
pub fn int_expr_check(analyzer: &Analyzer, lor_expr: &LOrExpr) -> Result<(), Error> {
  Ok(())
}

/// 检查表达式是否合法及其返回类型是否为 bool 类型。
/// 由于目前 Base Type 只有 int(i32)，bool 类型实际上并不存在而是用 int 代替，
/// 相当于检查返回类型是否为 int 类型，但为了区分和便于扩展还是单独定义一个函数。
pub fn bool_expr_check(analyzer: &Analyzer, lor_expr: &LOrExpr) -> Result<(), Error> {
  Ok(())
}
