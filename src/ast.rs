/*
CompUnit  ::= FuncDef;

FuncDef   ::= FuncType IDENT "(" ")" Block;
FuncType  ::= "midi" | "mp3";

Block     ::= "{" [Stmt] "}";
Stmt      ::= "play(" Number "," Number "," Number "," Number ");";
Number    ::= INT_CONST;
*/

#[derive(Debug)]
pub struct CompUnit {
  pub tempo: i32,
  pub time_signature: TimeSignature,
  pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {  // return type
  Midi,
  Mp3,
}

#[derive(Debug)]
pub struct Block {
  pub stmts: Vec<Play>,
}

#[derive(Debug)]
pub struct TimeSignature {
  pub numerator: i32,
  pub denominator: i32,
}

#[derive(Debug)]
pub struct Play {
  pub bar: i32,
  pub start: i32,
  pub end: i32,
  pub pitch: i32,
}
