fn main() {
    let hashes_width = [6, 1, 1, 5, 1, 1, 1];

    for width in hashes_width.iter()
    {
        for _i in 0..*width
        {
            print!("#");
        }
        println!();
    }
}
