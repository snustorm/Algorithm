mod Algorithm;
use Algorithm::binary_search::*; // Use the function directly if it's named `recursion`
use Algorithm::sorting::*;

fn main() {


    println!("Hello, world!");

    let mut arr = [3, 1, 1, 2, 1, 1, 0];
    let arr_string = ["aaa", "bbb", "abnc", "ddd", "zzz"];
    let result = bubble_sort(& arr_string);
    
    for i in result {
        print!("{} ", i);
    }


}
