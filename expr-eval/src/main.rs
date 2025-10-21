use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
enum ExprError {
  Parse(String),
}
impl std::fmt::Display for ExprError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Parse(s) => {
        write!(f, "{}", s)
      }
    }
  }
}
impl std::error::Error for ExprError {}

// 左结合
const ASSOC_LEFT: i32 = 0;
// 右结合
const ASSOC_RIGHT: i32 = 1;

#[derive(Debug, Clone, Copy)]
enum Token {
  Number(i32),
  Plus,
  Minus,
  Multiply,
  Divide,
  Power,
  LeftParen,
  RightParen,
}
impl Token {
  fn is_operator(&self) -> bool {
    match self {
      Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power => true,
      _ => false,
    }
  }
  /// 获取运算符的优先级
  fn precedence(&self) -> i32 {
    match self {
      Token::Plus | Token::Minus => 1,
      Token::Multiply | Token::Divide => 2,
      Token::Power => 3,
      _ => 0,
    }
  }
  fn assoc(&self) -> i32 {
    match self {
      Token::Power => ASSOC_RIGHT,
      _ => ASSOC_LEFT,
    }
  }
  fn compute(&self, l: i32, r:i32) -> Option<i32> {
    match self {
      Token::Plus => Some(l + r),
      Token::Minus => Some(l - r),
      Token::Multiply => Some(l * r),
      Token::Divide => Some(l / r),
      Token::Power => Some(l.pow(r as u32)),
      _ => None,
    }
  }
}

struct Tokenizer<'a> {
  tokens: Peekable<Chars<'a>>,
}
impl<'a> Tokenizer<'a> {
  fn new(expr: &'a str) -> Self {
    Self {
      tokens: expr.chars().peekable(),
    }
  }
  fn trim_start(&mut self) {
    while let Some(c) = self.tokens.peek() {
      if c.is_whitespace() {
        self.tokens.next();
      } else {
        break;
      }
    }
  }
  fn scan_number(&mut self) -> Option<Token> {
    let mut num = String::new();
    while let Some(&c) = self.tokens.peek() {
      if c.is_numeric() {
        num.push(c);
        self.tokens.next();
      } else {
        break;
      }
    }
    match num.parse::<i32>() {
      Ok(n) => {
        Some(Token::Number(n))
      },
      Err(_) => {
        None
      }
    }
  }
  fn scan_operator(&mut self) -> Option<Token> {
    match self.tokens.next() {
      Some('+') => Some(Token::Plus),
      Some('-') => Some(Token::Minus),
      Some('*') => Some(Token::Multiply),
      Some('/') => Some(Token::Divide),
      Some('^') => Some(Token::Power),
      Some('(') => Some(Token::LeftParen),
      Some(')') => Some(Token::RightParen),
      _ => None,
    }
  }
}
impl<'a> Iterator for Tokenizer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    self.trim_start();
    if let Some(n) = self.scan_number() {
      return Some(n);
    }
    if let Some(o) = self.scan_operator() {
      return Some(o);
    }
    None
  }
}

struct Expr<'a> {
  iter: Peekable<Tokenizer<'a>>,
}
impl<'a> Expr<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      iter: Tokenizer::new(src).peekable(),
    }
  }
  pub fn eval(&mut self) -> Result<i32, ExprError> {
    self.compute_expr(1)
  }
  pub fn compute_atom(&mut self) -> Result<i32, ExprError> {
    match self.iter.peek() {
      Some(Token::Number(n)) => {
        let val = *n;
        self.iter.next();
        Ok(val)
      },
      Some(Token::LeftParen) => {
        self.iter.next();
        let result = self.compute_expr(1)?;
        match self.iter.next() {
          Some(Token::RightParen) => (),
          _ => {
            return Err(ExprError::Parse("Unexpected character".into()));
          },
        }
        Ok(result)
      },
      _ => {
        Err(ExprError::Parse("compute_atom".into()))
      },
    }
  }
  pub fn compute_expr(&mut self, min_prec: i32) -> Result<i32, ExprError> {
    let mut atom_lhs = self.compute_atom()?;

    loop {
      let token = match self.iter.peek() {
        Some(t) => *t,
        None => break,
      };
      if !token.is_operator() || token.precedence() < min_prec {
        break;
      }
      let mut next_prec = token.precedence();
      if token.assoc() == ASSOC_LEFT {
        next_prec += 1;
      }
      self.iter.next();

      // 递归调用
      let atom_rhs = self.compute_expr(next_prec)?;
      match token.compute(atom_lhs, atom_rhs) {
        Some(n) => {
          atom_lhs = n;
        },
        None => {
          return Err(ExprError::Parse("compute Error".into()));
        },
      }

    }

    Ok(atom_lhs)
  }
}

fn main() {
  let src = "1 + (100 + 50) * 2 ^ 2";
  let mut expr = Expr::new(src);
  println!("{}", expr.eval().unwrap());
}
