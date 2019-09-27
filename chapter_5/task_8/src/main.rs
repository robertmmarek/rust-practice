trait Mult{}

impl Mult for usize{
}

impl Mult for isize{
}

fn multiply<T: Mult+std::ops::Mul>(a: T, b: T) -> <T as std::ops::Mul>::Output{
    a*b
}

fn main() {
    let a_u: usize = 2;
    let b_u: usize = 3;
    let a_i: isize = 2;
    let b_i: isize = 3;
    println!("{}", multiply(a_u, b_u));
    println!("{}", multiply(a_i, b_i));
}
