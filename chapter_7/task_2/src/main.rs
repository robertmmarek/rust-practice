use std::vec;

fn main() {
    let mut numbers: vec::Vec<i32> = vec::Vec::new();

    for i in 0..100{
        numbers.push(i);
    }
    numbers.retain(|x| !(x%3 == 0));
    println!("{:?}", numbers);
}
