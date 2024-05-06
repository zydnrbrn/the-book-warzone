fn main() {
    println!("Hello, world!");
}

#[test]
fn hi_dad() {
    println!("Hi Dad !")
}

#[test]
fn variable() {
    let greeting = "Good Afternoon!";
    println!("Good Morning! and {}", greeting)
}

#[test]
fn mutable_variable() {
    let mut planets = "Earth";
    println!(
        "Before April 2024, weather temperature on {} is normal.",
        planets
    );
    planets = "Mercury";
    println!(
        "But on April, earth especially on SEA is like on {}, closely to sun 💀",
        planets
    );
}

#[test]
// We can't change name into another data type, because name has defined as string.
fn static_typing() {
    let mut name = "Zidan Khulul Sajid";
    println!("My name is {}", name);

    name = "zydnrbrn";
    println!("My username is {}", name);
}

#[test]
// Shadowing is redefining a variable with the same name, but this very bad practice if we're always use this.
fn shadowing() {
    let country = "Indonesia";
    println!("I'm from {}", country);
    let country = "Japan";
    println!("But I want to move to {}", country);
}

#[test]
fn explicit_type() {
    let number: i32 = 11;
    let float: f64 = 11.11;
    let boolean: bool = true;
    let character: char = 'A';
    let string: &str = "Hello, World!";
    println!(
        "Number: {}, Float: {}, Boolean: {}, Character: {}, String: {}",
        number, float, boolean, character, string
    );
}
