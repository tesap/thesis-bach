
use std::hint::black_box;

fn main() {
  // Error: isn't initialized
  // let ref: &i64;

  // 42 is implicitly placed on stack
  let mut r: &i64 = &42;

  let x: i64 = 10;
  r = &x;

  // Prints value of referred variable
  println!("value: {:}", r);
}
