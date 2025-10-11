use serde_json::Value;

pub enum PathPart<'a> {
  Key(&'a str),
  Index(usize),
}
impl<'a> From<usize> for PathPart<'a> {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}
impl<'a > From<&'a str> for PathPart<'a> {
    fn from(value: &'a str) -> Self {
        Self::Key(value)
    }
}

pub struct GetValue<'a> {
  current: &'a Value,
}
impl<'a> GetValue<'a> {
  pub fn new(root:&'a Value) -> Self {
    Self { current: root }
  }
  pub fn chain(&self, key: impl Into<PathPart<'a>>) -> Self {
    let key = key.into();
    match key {
      PathPart::Key(key) => {
        Self { current: &self.current[key] }
      },
      PathPart::Index(key) => {
        Self { current: &self.current[key] }
      },
    }
  }
  pub fn to_str(&self) -> &'a str {
    self.current.as_str().unwrap_or("None")
  }
}
