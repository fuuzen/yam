/// 每一种 Base Type 的默认初始值
pub const INT_DEFAULT: RVal = RVal::Int(0);

/// 所有的 Base Type 的定义
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BType {
  Int,
  Bool,
}

/// Left Value，左值
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LVal {
  Ident(String),
}

/// Rightt Value，右值，每一种 Base Type 的具体存储类型
#[derive(Debug, Clone)]
pub enum RVal {
  Int(i32),
}