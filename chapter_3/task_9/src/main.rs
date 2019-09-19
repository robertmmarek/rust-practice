use std::cell;

#[derive(Debug)]
struct Immutable{
    mutable: cell::Cell<i32>
}

fn main() {
    let a = Immutable { mutable: cell::Cell::new(1) };
    println!("{:?}", a);
    a.mutable.set(2);
    println!("{:?}", a);
}
