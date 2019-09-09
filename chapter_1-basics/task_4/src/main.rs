use std::io;

fn main() {
    let mut input_string = String::new();
    println!("Your name please: ");
    let stdin = io::stdin();
    stdin.read_line(&mut input_string).expect("Error during processing of input");

    input_string = input_string.trim().to_string();
    println!("Hello {}!", input_string);
}
