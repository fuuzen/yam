#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RetVal {
  Int(i32),
  Void,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Ctr {
  Return(RetVal),
  Continue,
  Break,
  None
}