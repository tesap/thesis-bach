
fn takes_ref(n: &i64) {
    // n is const
    println!("Inside takes_ref: {:}", n);
    println!("Inside takes_ref: {:}", *n);
}

fn main() {
    let a: i64 = 10;
    let b: &i64 = &a;
    let c: i64 = *b;
    println!("C: {:?}", c);


    takes_ref(b);

    takes_ref(&c);

    let d: &&i64 = &b;
    takes_ref(d);
    let d2: &&&i64 = &&b;
    takes_ref(d2);
}
