use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "chenjiyuan")]
pub struct Args {
  pub input: String,
}
