fn main() {


    let x: (i32, f64, u8) = (200, 1.2, 2);

    let two_thousand = x.0;

    let one_point_two = x.1;

    let two = x.2;

    println!("The values of the 'x' tuple are: x: {}, y: {}, z: {}", two_thousand, one_point_two, two);

}
