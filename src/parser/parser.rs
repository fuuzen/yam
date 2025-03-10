use lalrpop_util::lalrpop_mod;

lalrpop_mod!(yam);

pub struct parser {
  input: String
}

impl parser {
  pub fn parse() -> Track {
    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast: Track = yam::TrackParser::new().parse(&input).unwrap();

    // 输出解析得到的 AST
    println!("{:#?}", ast);

    ast
  } 
}
