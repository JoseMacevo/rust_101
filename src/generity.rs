fn greater<T: PartialOrd + Copy>(n: &Vec<T>) -> T {
    let mut max: T = n[0];
    for &i in n.iter() {
        if i > max {
            max = i;
        }
    }
    max
}

pub fn run() {
    let v: Vec<i32> = vec![1, 2, 7, 8, 123, 4];
    let max = greater(&v);
    println!("{}", max);
}
