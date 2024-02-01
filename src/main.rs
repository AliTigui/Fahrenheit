use std::io;
fn main() {
    println!("hello");
    println!("Type 1 to convert from celsius to fahrenheit");
    println!("Type 2 to convert from fahernheit  to celsius");
    let mut input: String = String::new();
    let mut degree: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read value");
    let input: u8 = input.trim().parse().expect("couldn't parse value");
    println!("enter value");
    io::stdin()
        .read_line(&mut degree)
        .expect("couldn't read value");
    let mut degree: f32 = degree.trim().parse().expect("couldn't parse value");
    degree = if input == 1 {
        to_fahrenheit(degree)
    } else {
        to_celsius(degree)
    };
    println!("result is {degree}");
}
// function start here 
fn to_fahrenheit(degree: f32) -> f32 {
    degree * 9.0 / 5.0 + 32.0
}
fn to_celsius(degree: f32) -> f32 {
    degree - 32.0 * 5.0 / 9.0
}
