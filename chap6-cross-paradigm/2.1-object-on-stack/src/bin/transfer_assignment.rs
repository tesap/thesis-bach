use object_as_resource_owner::File;

fn main() {
    let mut f1 = File::new("file1.txt");

    let f_copy = f1.clone(); // Copy emulated
    let f_move = f1; // Implicit move

    // Use moved object
    println!("{}", f_move.fd);
}
