use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use yam::Parser;
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
  let parser = Parser::new();
  let parse_res = parser.parse(&input);
  if parse_res.is_err() {
    println!("{}", parse_res.unwrap_err());
    return Ok(());
  }
  let content = format!("{:#?}", parse_res.unwrap());
  let mut file = File::create(output)?;
  file.write_all(content.as_bytes())?;

  Ok(())
}
