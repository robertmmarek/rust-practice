
#[derive(Debug)]
struct ComplexNumber{
    real: f32,
    imag: f32
}

impl std::convert::From<(f32, f32)> for ComplexNumber{
    fn from(other: (f32, f32)) -> Self{
        ComplexNumber { real: other.0, imag: other.1 }
    }
}

fn main() {
    let a = ComplexNumber::from((1., 2.));
    let b: ComplexNumber = (3., 4.).into();
    println!("{:?}", a);
    println!("{:?}", b);
}
