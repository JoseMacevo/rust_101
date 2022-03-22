struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    origin: Poin,
    width: i32,
    height: i32,
}

pub fn run() {
    let p = Point { x: 50, y: 50 };
    println!("Punto X: {}", p.x);
}
