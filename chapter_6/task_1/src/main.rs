use std::vec;
use std::ops;

#[derive(Debug)]
struct Set{
    data: vec::Vec<i32>
}


impl ops::Sub<Set> for Set{
    type Output = Set;

    fn sub(mut self, other: Set) -> Set {
        for el in other.data.iter(){
            match self.data.iter().position(|&x| x == *el){
                Some(x) => {self.data.remove(x);},
                None => {();}
            };
        }
        Set {data: self.data.clone()}
    }
}

fn main() {
  let mut first = Set { data: vec![1, 2, 3, 4] };
  let mut second = Set { data: vec![3, 4, 5, 6] };

  println!("{:?}", first-second);
}
