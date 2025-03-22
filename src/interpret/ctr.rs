#[derive(PartialEq, Eq, Clone)]
pub enum RetVal {
  Int(i32),
  Void,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Ctr {
  Return(RetVal),
  Continue,
  Break,
  None
}