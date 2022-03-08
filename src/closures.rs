pub fn run() {
    let potencia_sup = |x| {
        let y = x + 1;
        y * y
    };
    let pot_5 = potencia_sup(5);
    println!("{}", pot_5);
}
