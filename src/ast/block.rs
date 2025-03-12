use std::sync::atomic::{AtomicU64, Ordering};

use super::stmt::Stmt;

pub type BlockId = u64;

pub static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// 追踪 Block 作用域内所有声明的函数、常量、变量的值的 Block 的状态
#[derive(Debug)]
pub struct Block {
  pub stmts: Vec<Stmt>,
  
  /// 标识每一个 Block 的唯一 ID，parse 阶段原子自增给出
  pub block_id: BlockId,
  
  /// parse 阶段还无法给出，只能在语义检查阶段找到其父 Block
  pub parrent_id: Option<BlockId>,
  
  /// 表明这是一个 while 循环的 Block。目前将该属性设置放在语义检查阶段
  pub while_: bool,
  
  /// 表明这是一个函数定义的 Block，它的 Identity。目前将该属性设置放在语义检查阶段
  pub func: Option<String>,
}

impl Block {
  pub fn new(stmts: Vec<Stmt>) -> Self {
    Block {
      stmts,
      block_id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
      parrent_id: None,
      while_: false,
      func: None,
    }
  }

  /// 获取 block_id
  pub fn get_id(&self) -> BlockId {
    self.block_id
  }

  /// 定义父 Block 的 id
  pub fn set_parrent_id(&mut self, id: BlockId) {
    self.parrent_id = Some(id);
  }

  /// 定义父 Block 属于一个函数
  pub fn set_func(&mut self, ident: String) {
    self.func = Some(ident);
  }

  /// 定义父 Block 属于一个 while 循环
  pub fn set_while(&mut self) {
    self.while_ = true;
  }
}

