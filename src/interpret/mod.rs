pub mod calc;

// use midi_file::MidiFile;
// use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
// use midi_file::file::{QuartersPerMinute, Track};

use std::{collections::HashMap, rc::Rc};

use crate::ast::btype::RVal;
use crate::ast::func::FuncFParam;
use crate::error::Error;

use crate::{ast::{block::{Block, BlockId}, func::FuncCall, track::Track}, semantic::{blocks::Blocks, scope::{BlockScope, Scopes}, symbol::Symbol}};

/// 解释器
pub struct Interpreter {
  /// 指向当前分析检查到的 Block 的 Id
  current_block_id: BlockId,

  /// 根据 BlockId 快速定位到 Rc<Block>
  block_table: HashMap<BlockId, Rc<Block>>,

  /// 根据 BlockId 快速定位到 BlockScope
  scope_table: HashMap<BlockId, BlockScope>,

  /// int 类型符号和他的具体值的哈希表
  int_table: HashMap<Symbol, i32>,
}

impl Interpreter {
  pub fn new(blocks: Blocks, scopes: Scopes) -> Self {
    Self {
      current_block_id: 0,
      block_table: blocks.get_block_table().clone(),
      scope_table: scopes.get_scope_table().clone(),
      int_table: HashMap::new(),
    }
  }

  /// 根据给定 BlockId 获取 Rc<Block>
  pub fn get_block(&self, block_id: BlockId) -> Result<Rc<Block>, Error> {
    Ok(self.block_table.get(&block_id).ok_or_else(|| Error::RuntimeError(format!("can't get this block: {}", block_id))).unwrap().clone())
  }

  /// 根据给定 BlockId 获取 BlockScope
  pub fn get_scope(&self, block_id: BlockId) -> Result<BlockScope, Error> {
    Ok(self.scope_table.get(&block_id).ok_or_else(|| Error::RuntimeError(format!("can't get BlockScope of this block: {}", block_id))).unwrap().clone())
  }

  /// 获取指定非函数 Symbol 的值
  pub fn get_val(&self, symbol: &Symbol) -> Result<RVal, Error> {
    let Symbol{ const_, btype, func_def, block_id } = symbol;
    
  }

  /// 进行赋值操作
  pub fn asgn(&mut self, symbol: &Symbol, value: RVal) -> Result<(), Error> {
    match value {
      RVal::Int(val) => {
        self.int_table.entry(symbol.clone()).and_modify(|v| *v = val).or_insert(val);
      }
    }
    Ok(())
  }

  /// 执行一段返回结果为 int 类型的函数
  pub fn call_int_func(&mut self, func_call: &FuncCall) -> Result<i32, Error> {
    let mut cur_block_id = self.current_block_id;

    let mut res_scope = self.get_scope(cur_block_id);
    if res_scope.is_err() {
      return Err(res_scope.err().unwrap());
    }
    let mut scope = res_scope.unwrap();
    
    let mut res_symbol = scope.get_symbol(&func_call.ident);

    let mut res_block = self.get_block(cur_block_id);
    if res_block.is_err() {
      return Err(res_block.err().unwrap());
    }
    
    let mut parent_block_id_  = res_block.unwrap().parrent_id;

    while res_symbol.is_none() && parent_block_id_.is_some() {
      cur_block_id = parent_block_id_.unwrap();
  
      res_scope = self.get_scope(cur_block_id);
      if res_scope.is_err() {
        return Err(res_scope.err().unwrap());
      }
      scope = res_scope.unwrap();
      
      res_symbol = scope.get_symbol(&func_call.ident);

      res_block = self.get_block(cur_block_id);
      if res_block.is_err() {
        return Err(res_block.err().unwrap());
      }
      
      parent_block_id_  = res_block.unwrap().parrent_id;
    }

    if res_symbol.is_none() {
      /* 不应该发生，在语义检查阶段就被排除 */
      return Err(Error::RuntimeError(format!("can't find this func: {}", func_call.ident)));
    }
    let symbol = res_symbol.unwrap().clone();
    
    let func_def = symbol.func_def.as_ref().unwrap();
    for i in 0..func_def.func_fparams.len() {
      let param = &func_def.func_fparams[i];
      let FuncFParam{btype, ident} = param;
      let expr = &func_call.func_rparams[i];
    }
    // self.current_block_id = ;

    Ok(0)
  }

  pub fn interpret_track(&self, track: &Track) -> () {
    // let mut midi_file = MidiFile::new();
    // let mut track = Track::default();
    // track.push_tempo(
    //   0,
    //   QuartersPerMinute::new(ast.tempo as u8)
    // ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // track.push_time_signature(
    //   0,
    //   ast.time_signature.numerator as u8,
    //   match ast.time_signature.denominator {
    //     1 => DurationName::Whole,
    //     2 => DurationName::Half,
    //     4 => DurationName::Quarter,
    //     8 => DurationName::Eighth,
    //     16 => DurationName::Sixteenth,
    //     32 => DurationName::D32,
    //     64 => DurationName::D64,
    //     128 => DurationName::D128,
    //     256 => DurationName::D256,
    //     512 => DurationName::D512,
    //     1024 => DurationName::D1024,
    //     _ => DurationName::Quarter, // TODO handle parse err
    // },
    //   Clocks::Quarter
    // ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // for (i, play) in ast.func_def.block.stmts.iter().enumerate() {
    //   let note_number = NoteNumber::new(60 + play.pitch as u8);
    //   let channel = Channel::new(0);
    //   let velocity = Velocity::new(72);
    //   let tick = 4 * 1024 / ast.time_signature.denominator;  // 默认 Divison (PPQ) = 1024
    //   let on_delta_time = if i == 0 {
    //     (tick * play.start) as u32  
    //   } else {
    //     ((play.bar - ast.func_def.block.stmts[i-1].bar) * 4 + play.start - ast.func_def.block.stmts[i-1].end) as u32
    //   };
    //   let off_delta_time = (tick * (play.end - play.start)) as u32;
    //   track.push_note_on(
    //     on_delta_time,  // 紧接着上一个事件
    //     channel,
    //     note_number,
    //     velocity
    //   ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    //   track.push_note_off(
    //     off_delta_time,
    //     channel,
    //     note_number,
    //     velocity
    //   ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // }

    // midi_file.push_track(track)
    //   .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // midi_file.save(output).unwrap();
    ()
  }
}