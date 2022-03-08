pub fn run() {
    // Clone
    let s1 = String::from("Bye, Bye - Xavier Cugat");
    let s2 = s1.clone();
    println!("{}", s1); // ERROR
}
// Prestamos con permiso de edicion (&mut)
fn f(s: &mut String) {
    s.push_str(" & Bye, Bye, Xavier Cugat");
}

fn z() {
    let mut s1 = String::from("Bolero - Maurice Ravel");
    f(&mut s1);
    println!("{}", s1);
}
// Prestamo con permiso de solo lectura.(&)
fn x() {
    let s1 = String::from("Bolero - Maurice Ravel");
    n(&s1);
    println!("{}, s1");
}
