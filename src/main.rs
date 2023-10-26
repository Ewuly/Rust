 fn main() {
    let bar = foo();
    println!("{bar}");
} 

fn foo() -> String {
    let s = String::from("Ewan");
    s
}

