use std::fs::read_to_string;
use std::io::Result;
use yam::{SyntacticAnalyzer, SemanticAnalyzer, Interpreter};

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// 输入文件路径
  #[arg(short = 'i', long = "input", required = true)]
  input: String,

  /// 输出文件路径
  #[arg(short = 'o', long = "output", required = true)]
  output: String,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let input = args.input;
  let output = args.output;

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
  println!("Lexical and syntactic parsed successfully");
  let comp_unit = parse_res.as_ref().unwrap();

  // 创建语义分析器
  let mut semantic_analyzer = SemanticAnalyzer::new();

  // 语义检查
  let res = semantic_analyzer.check(comp_unit);
  if res.is_err() {
    println!("{}", res.err().unwrap());
    return Ok(());
  }
  println!("Semantic check successflly");

  // 执行翻译
  let mut interpreter = Interpreter::new();
  let res = interpreter.interpret(comp_unit);
  if res.is_err() {
    println!("{}", res.err().unwrap());
    return Ok(());
  }
  println!("Interpret successflly");

  // 保存 midi 文件
  let res = res.unwrap().save(output);
  if res.is_err() {
    println!("{}", res.err().unwrap());
    return Ok(());
  }

  Ok(())
}
