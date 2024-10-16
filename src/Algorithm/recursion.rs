

pub fn recursion () {


    println!("Recursion Function");

}

//简单递归，斐波那契数列
pub fn fibonacci(n: u32) -> u32 {

    if n <= 1{
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

