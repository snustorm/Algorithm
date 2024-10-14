mod Algorithm;

use Algorithm::recursion::*; // Use the function directly if it's named `recursion`


fn main() {


    println!("Hello, world!");
    
    recursion();

    let number = 20;

    for i in 0..number {
        println!("fibonacci, {}: {}", i, fibonacci(i));
    }

}
