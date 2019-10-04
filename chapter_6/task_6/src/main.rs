use std::ops;

struct Hero{
    name: String
}

impl ops::Drop for Hero{
    fn drop(&mut self){
        println!("{}", &self.name[..]);
    }
}


fn main() {
   let a: Hero = Hero{ name: String::from("BATMAN") };
}
