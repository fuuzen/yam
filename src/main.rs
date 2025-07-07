use std::fs::read_to_string;
use std::io::{Result, Error, ErrorKind};
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
  let comp_unit =  parser.parse(&input)?;
  println!("Lexical and syntactic parsed successfully");

  // 创建语义分析器
  let mut semantic_analyzer = SemanticAnalyzer::new();

  // 语义检查
  semantic_analyzer.check(&comp_unit)?;
  println!("Semantic check successflly");
  
  // 创建解释器
  let mut interpreter = Interpreter::new();

  // 执行翻译
  let midi_file = interpreter.interpret(&comp_unit)?;
  println!("Interpret successflly");

  // 保存 midi 文件
  midi_file.save(output).map_err(|e|
    Error::new(ErrorKind::Other, e.to_string())
  )
}
