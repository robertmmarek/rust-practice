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

fn main() {
    let fib = Fibonacci::new();

   for i in fib.take(11){
       println!("{}", i);
   }
}
