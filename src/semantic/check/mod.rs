pub mod expr_check;
pub mod decl_check;
pub mod func_check;
pub mod stmt_check;
pub mod asgn_check;
pub mod lval_check;
pub mod while_check;
pub mod ifelse_check;
pub mod block_check;
pub mod asgn_rval_check;

use crate::{ast::comp_unit::CompUnit, error::Error};

pub use super::Analyzer;

impl Analyzer {
  /// 以 comp_unit 为单位进行语义检查。
  pub fn check(&mut self, comp_unit: &CompUnit) -> Result<(), Error> {
    // 进行 Block 为单位的语义检查
    self.global_block_check(comp_unit.block.clone())?;
    
    // 对 Score 进行语义检查
    self.score_check(&comp_unit.score)
  }
}