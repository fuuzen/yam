use lalrpop_util::lalrpop_mod;

use crate::ast::track::Track;

lalrpop_mod!(yam);

pub struct Parser {
  input: String
}

impl Parser {
  pub fn parse(input: String) -> Track {
    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast: Track = yam::TrackParser::new().parse(&input).unwrap();

    // 输出解析得到的 AST
    println!("{:#?}", ast);

    ast
  } 
}
