

// pub fn calculate() {
//     F(n) = F(n-1) + F(n-2)
// }

pub fn recursion () {


    println!("Recursion Function");

}

pub fn fibonacci(n: u32) -> u32 {

    if n <= 1{
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

