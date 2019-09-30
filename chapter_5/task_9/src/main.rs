trait Greatable{}


impl Greatable for i32{}
impl Greatable for f32{}

fn greater<'a, T: Greatable+std::cmp::PartialOrd>(a: &'a T, b: &'a T) -> &'a T{
    if a > b{
        a
    }else{
        b
    }
}

fn main() {
    let a_i: i32 = 5;
    let b_i: i32 = 6;
    let a_f: f32 = 5.0;
    let b_f: f32 = 6.0;
    println!("{}", greater(&a_i, &b_i));
    println!("{}", greater(&a_f, &b_f));
}
