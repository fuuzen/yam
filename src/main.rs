use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

// 引用 lalrpop 生成的解析器，模块名是 .lalrpop 文件的名字
lalrpop_mod!(yam);

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let ast = yam::CompUnitParser::new().parse(&input).unwrap();

  // 输出解析得到的 AST
  println!("{:#?}", ast);
  Ok(())
}
