use std::fmt;

struct Table<T: ?Sized>{
    legs_info: T
}

fn display_leg_info(el: &Table<fmt::Display>){
    println!("{}", el.legs_info.to_string());
}


fn main() {
    let a: Table<String> = Table { legs_info: String::from("abc") };
    let b: Table<u64> = Table { legs_info: 42 };
    display_leg_info(&a);
    display_leg_info(&b);
}
