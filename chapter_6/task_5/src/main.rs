use std::ops;

struct I32Wrapper(i32);

impl ops::Add<f32> for I32Wrapper{
    type Output = i32;
    fn add(self, other: f32) -> Self::Output{
        self + ((other as i32)) as f32
    }
}

fn main() {
   let a: i32 = 15;
   let b: f32 = 14.0;

   println!("{}", a+b);
}
