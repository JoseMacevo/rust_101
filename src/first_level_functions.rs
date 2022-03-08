fn ladrar() {
    println!("Guau, Guau, Guau....GRRRR");
}

fn doit_n_times(f: fn(), n: i64) {
    for _ in 0..n {
        f();
    }
}

pub fn run() {
    doit_n_times(ladrar, 42);
}
