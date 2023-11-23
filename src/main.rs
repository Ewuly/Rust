#[allow(dead_code)]
enum Continent {
    Europe,
    Asia,
    Africa,
    // ...
}

fn main() {
    let continent = Continent::Africa;

    match continent {
        Continent::Europe => print!("EU"),
        Continent::Asia => print!("A"),
        Continent::Africa => print!("AF"),
    }

}