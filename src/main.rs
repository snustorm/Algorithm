mod Algorithm;
use Algorithm::binary_search::*; // Use the function directly if it's named `recursion`
use Algorithm::sorting::*;

fn main() {


    println!("Hello, world!");

    let mut arr = [3, 1, 1, 2, 1, 1, 0, 1];
    let arr_string = ["aaa", "bbb", "abnc", "ddd", "zzz"];
    
    let result = merge_sort(&arr);

    for i in result {
        println!("{}", i);
    }


}
