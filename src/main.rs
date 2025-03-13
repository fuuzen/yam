use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use yam::{SyntacticAnalyzer, SemanticAnalyzer};
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

  // 创建词法&语法分析器
  let parser = SyntacticAnalyzer::new();

  // 词法&语法解析
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

  // 创建语义分析器
  let mut semantic_analyzer = SemanticAnalyzer::new();

  // 语义检查
  let res = semantic_analyzer.track_check(track);
  if res.is_err() {
    println!("{}", res.err().unwrap());
    return Ok(());
  }
  println!("Semantic check successflly");

  // 解释执行
  let interpreter = {
    let (blocks, scopes) = res.unwrap();
    yam::Interpreter::new(blocks, scopes)
  };

  Ok(())
}
