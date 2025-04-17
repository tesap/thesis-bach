use object_as_resource_owner::File;

pub fn foo(f: File) {
    println!("__FD: {}", f.fd);
}

pub fn general(val: File) {
    foo(val);
}

pub fn main() {
    let f1: File = File::new("file1.txt");
    println!("__FD (f1): {}", f1.fd);
    general(f1.clone()); // Copy
    general(f1); // Move
}

