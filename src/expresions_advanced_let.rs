pub fn run() {
    let age = 65;
    let x = if age > 18 {
        "Mayor de edad"
    } else {
        "Menor de edad"
    };
    println!("{}", x);
    second_type();
}

fn second_type() {
    let age = 65;
    let a = 10;
    let b = 25;
    let x = {
        let u = a * b;
        u + age
    };
    println!("{}", x);
}
