fn min_of_two<T: Ord>(a: T, b: T) -> T{
    if a >= b{
        b
    }else{
        a
    }
}

fn main() {
    println!("3, 4: {}", min_of_two(3, 4));
}
