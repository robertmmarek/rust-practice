#[derive(Debug, Copy, Clone)]
struct ComplexNumber{
    re: f32,
    im: f32
}

impl std::ops::Sub<ComplexNumber> for ComplexNumber{
    type Output = ComplexNumber;
    fn sub(self, other: ComplexNumber) -> ComplexNumber{
        ComplexNumber { re: self.re-other.re, im: self.im-other.im }
    }
}

impl std::ops::Add<ComplexNumber> for ComplexNumber{
    type Output = ComplexNumber;
    fn add(self, other: ComplexNumber) -> ComplexNumber{
        ComplexNumber { re:self.re+other.re, im:self.im+other.im }
    }
}

fn main() {
    let a = ComplexNumber{ re: 1., im: 2. };
    let b = ComplexNumber{ re: 3., im: 4. };
    println!("a+b: {:?}", a.clone()+b.clone());
    println!("a-b: {:?}", a.clone()-b.clone());
}
