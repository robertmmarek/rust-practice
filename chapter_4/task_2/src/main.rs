#[derive(Debug)]
enum Human{
    adult,
    child
}

impl Human{
    fn new(age: u32) -> Human{
        if age >= 18{
            Human::adult
        }else{
            Human::child
        }
    }
}

fn main() {
    let child = Human::new(6);
    let adult = Human::new(19);
    println!("{:?}", child);
    println!("{:?}", adult);
}
