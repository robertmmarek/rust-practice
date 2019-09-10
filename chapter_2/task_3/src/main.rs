fn main() {
    let base: i32 = 9;
    let middle: i32 = base/2;

    for i in 0..5
    {
        for j in 0..base
        {
            if j >= middle-i && j <= middle+i
            {
                print!("{}", '*');
            }else{
                print!("{}", ' ');
            }
        }
         println!();
    }
}
