fn swap(a: &mut i32, b: &mut i32)
{
    let cpy = *b;
    *b = *a;
    *a = cpy;
}

fn main() {
    let mut a = 5;
    let mut b = 6;
    swap(&mut a, &mut b);

    println!("a:{}, b:{}", a, b);
}
