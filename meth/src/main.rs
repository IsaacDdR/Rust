struct Point {
    x: f64,
    y: f64,
}


impl Point {
    fn orign -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new (x: f64, y: f64) -> Point {
        Point { x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {

    }

}