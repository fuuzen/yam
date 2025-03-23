pub mod expr_check;
pub mod decl_check;
pub mod func_check;
pub mod stmt_check;
pub mod asgn_check;
pub mod lval_check;
pub mod while_check;
pub mod ifelse_check;
pub mod block_check;

use crate::{ast::comp_unit::CompUnit, error::Error};

pub use super::Analyzer;

impl Analyzer {
  /// 以 comp_unit 为单位进行语义检查。
  /// 检查无误后返回 Blocks 和 Scopes 供解释器读取。
  /// 全局变量常量的作用域视为一个 Block，这个 Block 没有父级 Block，不设置其 parent_id。
  pub fn check(&mut self, comp_unit: &CompUnit) -> Result<(), Error> {
    // 进行 Block 为单位的语义检查
    let res = self.global_block_check(comp_unit.block.clone());
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    Ok(())
  }
}