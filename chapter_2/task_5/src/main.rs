fn factorial(n: u32) -> u32{
    if n == 0{
        1
    }else{
        n*factorial(n-1)
    }
}


fn main() {
    println!("factorial of 3: {}", factorial(3));
    println!("factorial of 1: {}", factorial(1));
    println!("factorial of 6: {}", factorial(6));
}
