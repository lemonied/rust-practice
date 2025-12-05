mod serialization;

use serialization::SearchParams;

use crate::serialization::SearchHash;

struct Foo<'a> {
  bar: &'a str,
}
fn baz<'a>(f: &'a Foo) -> &'a str {
  f.bar
}

fn main() {
  let s = "?a=1&b=2";
  let sp = SearchParams { src: s };
  let result1 = sp.serialize::<&str>().unwrap_or_else(|e| e.text);
  let result2 = sp.serialize::<SearchHash>().unwrap();
  println!("result1: {}", result1);
  println!("result2: {}", result2);
  println!("a: {}", result2["a"]);
  println!("c: {}", result2["c"]);
  
}

#[cfg(test)]
mod tests {

  use crate::*;

  #[test]
  fn test_baz() {
    let f = Foo { bar: "123" };
    assert!(baz(&f) == "123");
  }

}
