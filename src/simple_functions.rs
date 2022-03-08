fn sumatory(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run() {
    let a = 5;
    let b = 42;
    let c = sumatory(a, b);
    println!("{}", c);
}
