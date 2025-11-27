fn main() {
  let mut v = vec![1, 2, 3];
  for i in &mut v {
    *i += 1;
  }
  println!("{:#?}", v);
}
