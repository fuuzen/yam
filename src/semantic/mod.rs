pub mod block;

use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::{btype::BType, track::Track};
use crate::error::Error;
use block::BlockState;

pub struct Analyzer {
  block_table: HashMap<u32, Rc<BlockState>>,
}

impl Analyzer {
  pub fn new() -> Self {
    Self {
      block_table: HashMap::new(),
    }
  }

  /// 分析 AST 并进行语义检查
  pub fn analyze(&self, track: &Track) -> Result<(), Error> {
    Ok(())
  }
}