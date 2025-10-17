use std::{ collections::HashMap, ops::Index};

#[derive(Debug)]
pub struct SerializeError<'a> {
  pub text: &'a str,
}
impl<'a> std::fmt::Display for SerializeError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}
impl<'a> std::error::Error for SerializeError<'a> {}

pub trait Serializable<'a>
where
  Self: Sized,
{
  fn transform(src: &'a str) -> Result<Self, SerializeError<'a>>;
}
impl<'a> Serializable<'a> for &'a str {
  fn transform(src: &'a str) -> Result<Self, SerializeError<'a>> {
    if src.starts_with("?") {
      Ok(&src[1..])
    } else {
      Err(SerializeError { text: "开头缺少?" })
    }
  }
}

#[derive(Debug)]
pub enum SearchHash<'a> {
  Map(HashMap<&'a str, Self>),
  String(&'a str),
  None,
}
impl<'a> SearchHash<'a> {
  pub fn new(src: &'a str) -> Result<Self, SerializeError<'a>> {
    let mut src = src;

    if src.starts_with("?") {
      src = &src[1..];
    } else {
      return Err(SerializeError { text: "开头缺少?" });
    }

    let mut hm = HashMap::new();
    for item in src.split("&").into_iter() {
      let mut kv = item.split("=");
      let k = kv.next().unwrap();
      let v = kv.next().unwrap();
      hm.insert(k, Self::String(v));
    }

    Ok(Self::Map(hm))
  }
}
impl<'a> Index<&str> for SearchHash<'a> {
  type Output = Self;

  fn index(&self, index: &str) -> &Self::Output {
    if let SearchHash::Map(m) = self {
      m.get(index).unwrap_or_else(|| &Self::None)
    } else {
      self
    }
  }
}
impl<'a> std::fmt::Display for SearchHash<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Map(h) => {
        write!(f, "{h:?}")
      },
      Self::String(s) => {
        write!(f, "{s}")
      },
      Self::None => {
        write!(f, "None")
      },
    }
  }
}
impl<'a> Serializable<'a> for SearchHash<'a> {
  fn transform(src: &'a str) -> Result<Self, SerializeError<'a>> {
    SearchHash::new(src)
  }
}

pub struct SearchParams<'a> {
  pub src: &'a str,
}
impl<'a> SearchParams<'a> {
  pub fn serialize<T>(&self) -> Result<T, SerializeError<'_>>
  where
    T: Serializable<'a>
  {
    T::transform(self.src)
  }
}
