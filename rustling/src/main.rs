fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0{
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is small");
            10 * n
        } else {
            println!(", and is big");
            n / 2
        };
    println!("{} -> {}", n, big_n);
}


