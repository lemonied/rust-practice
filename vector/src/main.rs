fn main() {
  let mut v = vec![1, 2, 3];
  for i in &mut v {
    *i += 1;
  }
  let v = vec![1, 2, 3];
  let a: Vec<_> = v.iter().filter(|i| *i % 2 != 0).collect();
  let b: Vec<_> = v.iter().map(|i| i + 1).filter(|i| i % 2 != 0).collect();
  println!("a: {:?}", a);
  println!("b: {:?}", b);
  println!("{:#?}", v);
}
