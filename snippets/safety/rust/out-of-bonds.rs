
fn main() {
    let arr = [1, 2, 3, 4, 5]; // Array of size 5
    let index = 10; // Out-of-bounds index

    // Using safe access with bounds checking
    match arr.get(index) {
        Some(&value) => println!("Value at index {}: {}", index, value),
        None => println!("Error: Index {} is out of bounds!", index),
    }
}
