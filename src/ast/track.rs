use crate::ast::{func::FuncCall, phrase::{PhraseRVal, PhraseValue}, val::LVal};


/// 代表一个轨道
#[derive(Debug)]
pub struct Track {
  pub content: Vec<PhraseRVal>,
}

/// 可以用于赋值给 Track 类型的右值
#[derive(Debug)]
pub enum TrackRVal {
  Track(Track),
  LVal(LVal),
  FuncCall(FuncCall)
}

/// 表达式都被计算好后的 Track 值
#[derive(Debug, Clone)]
pub struct TrackValue {
  pub content: Vec<PhraseValue>,
}