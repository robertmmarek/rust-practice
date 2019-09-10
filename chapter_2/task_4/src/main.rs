fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input);
    let read_int: i32 = match input.trim().to_string().parse::<i32>(){
        Ok(n) => n,
        Err(e) => panic!("error parsing"),
    };

    if read_int%2 == 0{
        println!("even");
    }else{
        println!("odd");
    }

}
