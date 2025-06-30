use crate::ast::{func::FuncCall, measure::{MeasureAttr, MeasureAttrValue, MeasureRVal, MeasureValue}, val::LVal};


/// 代表一个乐句，包括可选的小节属性，乐句的小节属性会作为所有包含的小节的默认属性，并被小节的属性覆盖。
#[derive(Debug)]
pub struct Phrase {
  pub attr: Option<MeasureAttr>,
  pub content: Vec<MeasureRVal>,
}

/// 可以用于赋值给 Phrase 类型的右值
#[derive(Debug)]
pub enum PhraseRVal {
  Phrase(Phrase),
  LVal(LVal),
  FuncCall(FuncCall)
}

/// 表达式都被计算好后的 Phrase 值
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhraseValue {
  pub attr: Option<MeasureAttrValue>,
  pub content: Vec<MeasureValue>,
}