pub fn run() {
    let tuple = (42, "Adrianistan", true);
    let (random, country, has_beers) = tuple;
    println!("{}", random);
    let (_, country, _) = tuple;
    println!("{}", country);
    let has_beers = tuple.2;
}
