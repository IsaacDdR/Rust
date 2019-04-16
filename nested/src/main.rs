#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outerloop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
