struct Addition<T>{
    a: T,
    b: T
}

impl<T> Addition<T>{
    pub fn new(a: T, b: T) -> Addition<T>{
        Addition {a, b}
    }
}

// impl<T: std::ops::Add + Copy + Clone> Addition<T>{
//     pub fn add(&self) -> <T as std::ops::Add>::Output{
//         self.a + self.b
//     }
// }

impl Addition<f32>{
    pub fn add(&self) -> f32{
        self.a + self.b
    }
}

impl Addition<i32>{
    pub fn add(&self) -> i32{
        self.a + self.b
    }
}

fn main() {
    let test = Addition::new(2.5, 3.0);
    println!("{}", test.add());
    let test = Addition::new(2, 3);
    println!("{}", test.add());
}
