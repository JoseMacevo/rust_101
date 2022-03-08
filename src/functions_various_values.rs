fn string_length_and_lines(txt: &String) -> (usize, usize) {
    (txt.len(), txt.lines().count())
}

pub fn run() {
    let s = String::from(
        "Europe's Skies - Alexander Rybak\nSuper Strut - Deodato\nEl Condor Pasa - Unha Ramos",
    );

    let (length, lines) = string_length_and_lines(&s);
    println!(
        "La lista de canciones tiene una longitud de {} caracteres y {} lineas ",
        length, lines
    );
}
