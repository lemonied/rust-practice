use std::{env, error::Error, fs, process};

use crate::utils::{config::Config, search::search_str};

mod utils;

fn run() -> Result<(), Box<dyn Error>> {
  let config = Config::new(env::args())?;
  println!("config: {}", config);

  let contents = fs::read_to_string(config.file_path)?;
  let result = search_str(&config.query, &contents);

  println!("result: {}", result.join("\n"));

  Ok(())
}

fn main() {
  run().unwrap_or_else(|err| {
    eprintln!("{}", err);
    process::exit(1);
  });
}
