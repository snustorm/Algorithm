mod Algorithm;

use Algorithm::binary_search::*; // Use the function directly if it's named `recursion`


fn main() {


    println!("Hello, world!");

    const arr: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
    let target = 'e';

    match binary_search(arr, 0, arr.len(), target) {
        Some(index) => println!("Found {} at the index {}", target, index),
        None => println!("'{}' not found in the array.", target),
    }


}
