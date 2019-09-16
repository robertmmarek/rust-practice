use std::ops;

#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f32,
    imag: f32
}

impl ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex{
        Complex {real: self.real+other.real, imag: self.imag+other.imag}
    }
}

impl ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex{
        Complex {real: self.real-other.real, imag: self.imag-other.imag}
    }
}

impl ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex{
        Complex {real: self.real*other.real - self.imag*other.imag, 
                 imag: self.real*other.imag+self.imag*other.real}
    }
}

fn main() {
    let a = Complex{real:2.0, imag:2.0};
    let b = Complex{real:3.0, imag:4.0};

    println!("add: {:?}", a+b);
    println!("sub: {:?}", a-b);
    println!("mul: {:?}", a*b);
}
