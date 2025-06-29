use std::rc::Rc;

use crate::ast::{block::Block, expr::Expr, track::TrackRVal};


/// 将指定 Channel 的输入设为一个 Track
#[derive(Debug)]
pub struct SetChannelTrack {
  pub channel: Expr,
  pub track: TrackRVal,
}

/// 设置指定 Channel 的 Midi 乐器
#[derive(Debug)]
pub struct SetChannelInstrument {
  pub channel: Expr,
  pub instrument: Expr,
}

/// 仅在 Score 的 block 中出现的 channel 相关操作
#[derive(Debug)]
pub enum ChannelStmt {
  SetChannelTrack(SetChannelTrack),
  SetChannelInstrument(SetChannelInstrument)
}

/// 代表一个乐谱，也是程序入口
#[derive(Debug)]
pub struct Score {
  pub block: Rc<Block>,
  pub channel_stmts: Vec<ChannelStmt>
}