use std::{cell::RefCell, rc::Rc};

fn main() {
  let n: Rc<i32> = Rc::new(1);
  let n2 = Rc::clone(&n);
  println!("n2: {}", n2);
  println!("strong_count: {}", Rc::strong_count(&n));

  let arr_rc = RefCell::new(vec![1, 2]);
  arr_rc.borrow_mut().push(3);
  println!("{:?}", arr_rc.borrow());
}
