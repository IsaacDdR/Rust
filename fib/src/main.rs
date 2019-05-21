fn fib(n: u32) -> u32{
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n -1 ) + (n - 2),
    }
}

fn main(){
    println!("Fibonacci generator");
    println!("{}", fib(1));
    println!("{}", fib(2));
    println!("{}", fib(221));
}

