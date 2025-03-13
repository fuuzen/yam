use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use yam::{Parser, SemanticAnalyzer};
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  // let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // 词法&语法解析
  let parser = Parser::new();
  let parse_res = parser.parse(&input);
  if parse_res.is_err() {
    println!("{}", parse_res.unwrap_err());
    return Ok(());
  }
  println!("Lexical syntactic parsed successflly");
  let track = parse_res.as_ref().unwrap();

  let content = format!("{:#?}", track);
  let mut file = File::create(output)?;
  file.write_all(content.as_bytes())?;

  // 语义检查
  let mut semantic_analyzer = SemanticAnalyzer::new();

  let res = semantic_analyzer.track_check(track);
  if res.is_err() {
    println!("{}", res.unwrap_err());
    return Ok(());
  }
  println!("Semantic check successflly");

  Ok(())
}
