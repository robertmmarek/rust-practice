struct Fibonacci{
    first: u32,
    current: u32,
    prev: u32
}

impl Fibonacci{
    fn new() -> Fibonacci{
        Fibonacci { first: 1, current: 1, prev: 0}
    }
}

impl Iterator for Fibonacci{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        let ret = self.current;
        self.current = ret + self.prev;
        self.prev = ret;
        
        Some(ret)
    }
}

fn sum<T: Iterator<Item = u32>>(it: T, max_index: usize) -> Option<u32>{
    let mut acc = 0;
    for el in it.take(max_index){
        acc += el;
    }
    Some(acc)
}

fn main() {
    let mut fib = Fibonacci::new();
    println!("{}", sum(fib, 4).unwrap());
}
