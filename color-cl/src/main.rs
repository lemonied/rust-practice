mod utils;

use std::process;

use clap::Parser;
use utils::arg::Args;

use utils::color::Color;

fn run() -> Result<(), String> {
  let args = Args::parse();
  let color = Color::parse(&args.input)?;
  println!("{}", color);

  Ok(())
}

fn main() {
  run().unwrap_or_else(|err| {
    eprintln!("{}", err);
    process::exit(1);
  });
}
