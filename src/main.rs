// generic : peut prendre different types
fn find_something<T: std::fmt::Display>(something: T) {
    println!("Votre something: {}", something);
}

fn main() {
    let my_number = find_something(7);
    let my_string = find_something(String::from("value"));
}