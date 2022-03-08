pub fn run() {
    let mut notas_array: [i32; 5] = [0; 5];
    notas_array[0] = 1;
    notas_array[1] = 6;

    let mut notas_vec: Vec<i32> = vec![];
    notas_vec.push(1);
    notas_vec.push(6);

    println!("Nota 2: {}", notas_array[1]);
    println!("Nota 2: {}", notas_vec[1]);
}
