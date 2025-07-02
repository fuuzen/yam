use crate::ast::{func::FuncCall, note::{Note, NoteValue}, val::LVal};


/// 小节的一个单元，每一个单元占一个小节的节拍类型分母所决定的音符长度
#[derive(Debug)]
pub enum MeasureUnit {
  /// '<' 时间变慢一倍,例如原来是四分音符将变为八分音符
  TimeDilation,

  /// '>' 时间变快一倍,例如原来是四分音符将变为二分音符
  TimeCompression,

  /// '.' 休止符
  Rest,

  /// 单个音符
  Note(Note)
}

/// 代表一个小节，包括可选的小节属性、小节的内容。
#[derive(Debug)]
pub struct Measure {
  pub content: Vec<MeasureUnit>,
}

/// 可以用于赋值给 Note 类型的右值
#[derive(Debug)]
pub enum MeasureRVal {
  Measure(Measure),
  LVal(LVal),
  FuncCall(FuncCall)
}


/// MeasureUnit 的 Value 版本
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasureUnitValue {
  /// '<' 时间变慢一倍,例如原来是四分音符将变为八分音符
  TimeDilation,

  /// '>' 时间变快一倍,例如原来是四分音符将变为二分音符
  TimeCompression,

  /// '.' 休止符
  Rest,

  /// 单个音符
  NoteValue(NoteValue)
}

/// 表达式都被计算好后的 Measure 值
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasureValue {
  pub content: Vec<MeasureUnitValue>,
}

impl Measure {
  pub fn a(&self) {
    // let a =  self.content.insert();
  }
}