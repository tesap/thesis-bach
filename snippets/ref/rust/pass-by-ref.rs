
use std::hint::black_box;

#[derive(Copy, Clone)]
struct A {
  pub x1: i64,
  pub x2: i64,
  pub x3: i64,
}

#[inline(never)]
fn as_val(mut inn: A) {
  inn.x1 += 100;
  inn.x2 += 100;
  inn.x3 += 100;
  black_box(&inn);
  //black_box(&inn.x1);
  //black_box(&inn.x2);
  //black_box(&inn.x3);
}

#[inline(never)]
fn as_ptr(inn: *mut A) {
    unsafe {
        (*inn).x1 += 100;
        (*inn).x2 += 100;
        (*inn).x3 += 100;
    }
    black_box(inn);
  //black_box(inn.x1);
  //black_box(inn.x2);
  //black_box(inn.x3);
}

#[inline(never)]
fn as_ref(inn: &mut A) {
  inn.x1 += 300;
  inn.x2 += 300;
  inn.x3 += 300;
  black_box(inn);
  //black_box(inn.x1);
  //black_box(inn.x2);
  //black_box(inn.x3);
}

fn main() {
  let mut a1 = A {
      x1: 10,
      x2: 20,
      x3: 30,
  };

  as_val(a1);
  as_ptr(&mut a1);
  as_ref(&mut a1);
  black_box(a1);
  black_box(a1.x1);
  black_box(a1.x2);
  black_box(a1.x3);
}
