use clap::Parser;

#[derive(Parser, Debug)]
#[command(
  name = "weather tool",
  version = "0.0.1",
  about = "一款查询天气的CLI工具"
)]
pub struct Args {
  /// 城市、地区名称
  #[arg(short, long)]
  pub city: Option<String>,
}
