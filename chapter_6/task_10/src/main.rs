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

trait Moddable{
    fn complex_mod(self) -> f32;
}

impl Moddable for ComplexNumber{
    fn complex_mod(self) -> f32{
        let add: f32 = (self.imag.powi(2)+self.real.powi(2));
        add.sqrt()
    }
}

impl Moddable for (f32, f32){
    fn complex_mod(self) -> f32{
        let complex = ComplexNumber::from(self);
        let add: f32 = (complex.imag.powi(2)+complex.real.powi(2));
        add.sqrt()
    }
}




fn main() {
    let a = ComplexNumber::from((1., 2.));
    println!("{}", a.complex_mod());
    println!("{}", (3., 4.).complex_mod());
}
