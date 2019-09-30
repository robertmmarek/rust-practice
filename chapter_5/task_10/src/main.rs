use std::fmt;

trait Hello{
    fn say(&self) where Self: std::fmt::Display{
        println!("Hello {}!", self);
    }
}

impl Hello for str{}
impl Hello for String{}

fn main() {
  "World".say();
  String::from("World2").say();
}
