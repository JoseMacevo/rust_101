fn search(t: (bool, &'static str, &'static str, i32)) -> &'static str {
    match t {
        (true, "Mike Oldfield", "The Bell", 1992) => "Great Song",
        (true, "Mike Oldfield", _, 1992) => "Tubular Bells II probably",
        (true, author, _, _) => author,
        (false, ..) => "Missing Disk",
        _ => "What's this?..",
    }
}

pub fn run() {
    let tuple = (true, "Mike Oldfield", "The Bell", 1992);
    let searching = search(tuple);
    println!("{}", searching);
}
