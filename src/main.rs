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

#[test]
fn calculating_number() {
    let first_number = 10;
    let second_number: i64 = 20;
    let result = first_number + second_number;

    println!("The result is {}", result);
}

#[test]
fn calculating_float() {
    let first_float = 10.5;
    let second_float = 20.7;

    let result = first_float + second_float;

    println!("The result is {}", result);
}

#[test]
fn number_conversion() {
    let first_number: i8 = 10;
    print!("The first number is {}", first_number);

    let converted_number: i16 = first_number as i16;
    println!(" and the converted number is {}", converted_number);

    let converted_number_second: i32 = converted_number as i32;
    print!("The converted number is {}", converted_number_second);
}

// This is absolutely wrong, and horrible. because we can't convert string to number directly.
#[test]
fn overflowing_number() {
    let normal_number: i64 = 1000000;
    print!(
        "This is the normal number, but we will try to overflowed it {}",
        normal_number
    );

    let overflowed_number: i8 = normal_number as i8;
    println!("This is the overflowed number {}", overflowed_number);
}

#[test]
fn number_following_os() {
    let number: usize = 100;
    println!("This number is following OS architecture, {}", number);

    let number: isize = -100;
    println!("This number is following OS architecture, {}", number);
}

#[test]
fn number_operator() {
    let number = 10;
    let number_1 = 20;
    let number_2 = 30;
    let number_3 = 40;
    let number_4 = 50;

    let result = number_4 - number * number_1 / number_2 % number_3;
    println!("The result is {}", result);
}

#[test]
fn augmented_assignment() {
    let mut number = 1;
    number += 100;
    number *= 200;
    number /= 10;
    number %= 3;

    println!("The result is {}", number);
}

#[test]
fn comparison_operator() {
    let current_time = 10.30;

    match current_time {
        0.0..=12.0 => println!("Good Morning!"),
        12.0..=18.0 => println!("Good Afternoon!"),
        18.0..=24.0 => println!("Good Evening!"),
        _ => println!("Good Night!"),
    }
}

#[test]
fn playing_with_tuple() {
    let mut tuple: (i64, usize, char) = (100, 198, 'Z');
    // this is destructuring tuple
    let (first, second, third) = tuple;

    println!("First: {}, Second: {}, Third: {}", first, second, third);

    // mutable tuple
    tuple.0 = 200;
    tuple.1 = 300;
    tuple.2 = 'A';

    println!(
        "First: {}, Second: {}, Third: {}",
        tuple.0, tuple.1, tuple.2
    );
}

#[test]
fn unit() {
    println!("Hello Dad!");
}

#[test]
fn test_unit() {
    let dad_greet = unit();
    println!("{:?}", dad_greet);
}

#[test]
fn array_playground() {
    // Array is just accept on data type
    let array = [1, 2, 3, 4];

    println!(
        "First: {}, Second: {}, Third: {}, Fourth: {}",
        array[0], array[1], array[2], array[3]
    );

    let mut array_mutable = [4, 6, 7, 3, 2, 9];

    println!("This is array before mutable {:?}", array_mutable);

    array_mutable[0] = 1;
    array_mutable[1] = 2;
    array_mutable[2] = 3;
    array_mutable[3] = 4;
    array_mutable[4] = 5;
    array_mutable[5] = 6;

    println!("This is array after mutable {:?}", array_mutable);
}

#[test]
fn two_dimentional_array() {
    let array: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("{:?}", array);
}

const MAXIMAL_NUMBER: i32 = 100;

#[test]
fn play_with_const() {
    const PI: f64 = 3.14;
    println!("The value of PI is {}", PI);
    println!("The maximal number is {}", MAXIMAL_NUMBER);
}
