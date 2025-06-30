use std::{cell::RefCell, fmt, rc::Rc};

use crate::ast::{measure::MeasureValue, note::NoteValue, phrase::PhraseValue, track::TrackValue};

/// 各个类型的默认初始值
pub const INT_DEFAULT: i32 = 0;
pub const NOTE_DEFAULT: NoteValue = NoteValue{notes: vec![], len: None};
pub const MEASURE_DEFAULT: MeasureValue = MeasureValue{attr: None, content: vec![]};
pub const PHRASE_DEFAULT: PhraseValue = PhraseValue{attr: None, content: vec![]};
pub const TRACK_DEFAULT: TrackValue = TrackValue{content: vec![]};

/// 所有的 Base Type 的定义
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BType {
  Int,
  Bool,
  Note,
  Measure,
  Phrase,
  Track,
}

impl fmt::Display for BType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BType::Int => write!(f, "int"),
      BType::Bool => write!(f, "bool"),
      BType::Note => write!(f, "note"),
      BType::Measure => write!(f, "measute"),
      BType::Phrase => write!(f, "phrase"),
      BType::Track => write!(f, "track"),
    }
  }
}

/// 所有的 Value 的定义
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  /// 也用作 bool
  Int(i32),
  Note(NoteValue),
  Measure(MeasureValue),
  Phrase(PhraseValue),
  Track(TrackValue),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Int(_) => write!(f, "i32"),
      Value::Note(_) => write!(f, "note"),
      Value::Measure(_) => write!(f, "measute"),
      Value::Phrase(_) => write!(f, "phrase"),
      Value::Track(_) => write!(f, "track"),
    }
  }
}

/// Left Value，左值
#[derive(Debug, Clone)]
pub struct LVal {
  pub ident: String,

  /// 语义检查阶段，绑定该左值的右值
  pub rval: Rc<RefCell<Option<Rc<RVal>>>>,
}

impl LVal {
  pub fn new(ident: String) -> Self {
    LVal {
      ident,
      rval: Rc::new(RefCell::new(None)),
    }
  }

  /// 具有类型 BType 的初始化，初始化为默认值
  pub fn new_with_btype(btype: BType, ident: String) -> Self {
    let rval = RVal::new_with_btype(btype);
    LVal {
      ident,
      rval: Rc::new(RefCell::new(Some(Rc::new(rval)))),
    }
  }

  /// 语义检查阶段绑定右值
  pub fn bind_rval(&self, rval: Rc<RVal>) {
    *self.rval.borrow_mut() = Some(rval);
  }

  /// 执行阶段获取值
  pub fn get_value(&self) -> Value {
    self.rval.borrow().as_ref().unwrap().get_value()
  }

  /// 赋值
  pub fn set_value(&self, value: Value) {
    self.rval.borrow().as_ref().unwrap().set_value(value);
  }
}

/// Rightt Value，右值，每一种 Base Type 的具体存储类型。
/// parse 阶段没有使用，语义检查阶段创建
#[derive(Debug, Clone)]
pub struct RVal {
  pub value: Rc<RefCell<Value>>,
}

impl RVal {
  /// 语义检查阶段统一初始化为默认值；实际运行时视初始化为普通的赋值。
  pub fn new_with_btype(btype: BType) -> Self {
    match btype {
      BType::Int => RVal{value: Rc::new(RefCell::new(Value::Int(INT_DEFAULT)))},
      BType::Note => RVal{value: Rc::new(RefCell::new(Value::Note(NOTE_DEFAULT)))},
      BType::Measure => RVal{value: Rc::new(RefCell::new(Value::Measure(MEASURE_DEFAULT)))},
      BType::Phrase => RVal{value: Rc::new(RefCell::new(Value::Phrase(PHRASE_DEFAULT)))},
      BType::Track => RVal{value: Rc::new(RefCell::new(Value::Track(TRACK_DEFAULT)))},
      _ => unimplemented!()
    }
  }

  /// 返回变量类型 Btype
  pub fn get_btype(&self) -> BType {
    match *self.value.borrow() {
      Value::Note(_) => BType::Note,
      Value::Int(_) => BType::Int,
      Value::Measure(_) => BType::Measure,
      Value::Phrase(_) => BType::Phrase,
      Value::Track(_) => BType::Track,
    }
  }

  /// 赋值
  pub fn set_value(&self, value: Value) {
    *self.value.borrow_mut() = value;
  }

  /// 获取 int 值
  pub fn get_value(&self) -> Value {
    self.value.borrow().clone()
  }
}