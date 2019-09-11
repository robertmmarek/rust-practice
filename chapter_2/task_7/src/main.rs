fn add_two(one: i32, two: i32) -> i32 {
    let addition = match one+two {
        -1 => panic!("Error adding!"),
        _ => one+two,
    };
    addition
}

fn main() {
    println!("{}", add_two(1, 2));
}
