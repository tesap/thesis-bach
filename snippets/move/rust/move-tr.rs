use std::hint::black_box;

fn main() {
    let s1 = String::from("hello");
    black_box(&s1);
    let s2 = s1;

    black_box(&s2);

    // println!("{s1}, world!");
    let x: i64 = 5;
    let y: i64 = x; // copy
    println!("x: {:}", x);
    println!("y: {:}", y);
}
