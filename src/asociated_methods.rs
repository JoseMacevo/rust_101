use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    origin: Point,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        self.area() == other.area()
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Rectangle) -> Option<Ordering> {
        if self.area() == other.area() {
            Some(Ordering::Equal)
        } else if self.area() > other.area() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Origin: ({}, {}) - Area:
        {}",
            self.origin.x,
            self.origin.y,
            self.area()
        )
    }
}

pub fn run() {
    let p = Point { x: 50, y: 50 };
    println!("Point X: {}", p.x);

    let r1 = Rectangle {
        origin: p,
        width: 20,
        height: 20,
    };
    println!("{}", r1);

    let r2 = Rectangle {
        origin: Point { x: 3, y: 4 },
        width: 30,
        height: 30,
    };
    println!("{}", r2);

    if r1 < r2 {
        println!("r2 is biger than r1");
    }
}
