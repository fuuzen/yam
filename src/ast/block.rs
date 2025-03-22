use std::{cell::RefCell, rc::Rc, sync::atomic::{AtomicU64, Ordering}};

use super::{func::FuncDef, stmt::Stmt};

pub type BlockId = u64;

/// Block 的 ID 自增器，从 0 开始。
pub static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Block 的 AST
#[derive(Debug)]
pub struct Block {
  pub stmts: Vec<Stmt>,
  
  /// 标识每一个 Block 的唯一 ID，parse 阶段原子自增给出
  pub block_id: BlockId,
  
  /// parse 阶段还无法给出，只能在语义检查阶段找到其父 Block
  /// 函数定义和 CompUnit 的 Block 的父级认为是没有的 
  pub parent_id: Rc<RefCell<Option<BlockId>>>,
  
  /// 表明这是一个 while 循环的 Block。目前将该属性设置放在语义检查阶段
  pub while_: Rc<RefCell<bool>>,
  
  /// 表明这是一个函数定义的 Block，它的 Identity。目前将该属性设置放在语义检查阶段
  pub func: Rc<RefCell<Option<Rc<FuncDef>>>>,
}

impl Block {
  pub fn new(stmts: Vec<Stmt>) -> Self {
    Block {
      stmts,
      block_id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
      parent_id:Rc::new(RefCell::new(None)),
      while_: Rc::new(RefCell::new(false)),
      func: Rc::new(RefCell::new(None)),
    }
  }

  /// 获取 block_id
  pub fn get_id(&self) -> BlockId {
    self.block_id
  }

  /// 获取 parent_id
  pub fn get_parent_id(&self) -> Option<BlockId> {
    self.parent_id.borrow().clone()
  }

  /// 定义父 Block 的 id
  pub fn set_parent_id(&self, id: BlockId) {
    *self.parent_id.borrow_mut() = Some(id);
  }

  /// 定义父 Block 所属于的函数
  pub fn set_func(&self, func_def: Rc<FuncDef>) {
    *self.func.borrow_mut() = Some(func_def);
  }

  /// 定义父 Block 属于一个 while 循环
  pub fn set_while(&self) {
    *self.while_.borrow_mut() = true;
  }
}
