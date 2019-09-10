fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error reading");
    let input_number: i32 = input.trim().parse::<i32>().expect("error parsing");

    match input_number{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("{}", input_number),
        _ => println!("NaN")
    }
}
