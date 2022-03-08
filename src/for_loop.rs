pub fn run() {
    let v = vec![
        "Haskell",
        "Elm",
        "Python",
        "C++",
        "Javascript",
        "Rust",
        "Java",
    ];

    for s in v {
        println!("{}", s);
    }

    println!("Printing numbers..");
    sjiecondecond();
    println!("Printing Numbers..");
    second();
}

fn second() {
    let n = 100;
    for i in 0..n {
        println!("{}", i);
    }
}
