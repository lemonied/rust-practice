use std::fmt::Display;

#[derive(Debug)]
pub enum ConfigError {
  Lack(String),
}
impl Display for ConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Lack(s) => {
        write!(f, "LackError: {}", s)
      }
    }
  }
}
impl std::error::Error for ConfigError {}

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {
  pub fn new<T>(mut args: T) -> Result<Self, ConfigError>
  where
    T: Iterator<Item = String>,
  {
    args.next();
    let file_path = args.next().ok_or(ConfigError::Lack("file_path is empty".to_owned()))?;
    let query = args.next().ok_or(ConfigError::Lack("query is empty".to_owned()))?;

    Ok(Self { file_path, query })
  }
}

impl Display for Config {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "query: {}, file_path: {}", self.query, self.file_path)
  }
}
