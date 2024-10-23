mod Algorithm;
use std::result;

use Algorithm::binary_search::*; // Use the function directly if it's named `recursion`
use Algorithm::sorting::*;

fn main() {


    println!("Hello, world!");

    let mut arr = [2, 6, 5, 3, 8, 7, 1, 0];
    //let mut arr = [8, 7, 6, 5];
    let arr_string = ["aaa", "bbb", "abnc", "ddd", "zzz"];
    
    let result = quick_sort(&arr);
    
    //let result = merge_sort(&arr);
    for i in result {
        println!("{}", i);
    }


}
