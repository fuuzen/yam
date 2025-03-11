mod block;

use crate::ast::{btype::BType, track::Track};
use crate::error::Error;

pub struct Analyzer {}

impl Analyzer {
  pub fn new() -> Self {
    Self {}
  }

  /// 分析 AST 并进行语义检查
  pub fn analyze(&self, track: &Track) -> Result<(), Error> {
    Ok(())
  }
}