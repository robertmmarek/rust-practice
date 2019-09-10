 fn main() -> Result<(), std::num::ParseIntError>{
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Problems during reading");

    input_text = input_text.trim().to_string();

    let int = match input_text.parse::<i32>()
    {
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    println!("{}", int);

    Ok(())
}


