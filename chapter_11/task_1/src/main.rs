macro_rules! addition {
    ($( $x:expr ),*) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    };
}

fn main() {
    println!("{}", addition!(5, 6, 57 ,56, 1));
}
