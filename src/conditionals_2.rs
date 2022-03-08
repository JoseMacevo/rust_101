pub fn run() {
    let url = Some("Https://blog.adrianistan.eu");

    if let Some(url) = url {
        println!("{}", url);
    }

    let magic_box: Result<String, String> = Ok(String::from("Aqui no hay nada"));
    if let Ok(magic_box) = magic_box {
        println!("{}", magic_box);
    }
    let url: Option<&'static str> = None;
    if let Some(url) = url {
        println!("{}", url);
    }

    let magic_box: Result<String, String> = Err(String::from("No tienes la llave de la caja"));
    if let Ok(magic_box) = magic_box {
        println!("{}", magic_box);
    }
}
