pub fn run() {
    let mut n = 0;
    loop {
        if n > 100 {
            break;
        }
        println!("I'm not heavy");
        n += 1
    }

    while n < 200 {
        println!("I'm not heavy");
        n += 1;
    }
}
