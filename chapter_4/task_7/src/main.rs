struct Point{
    x: f32,
    y: f32
}

fn main() {
    let point = Point{x: 1.0, y: 1.0};

    match point{
        Point{x: 0.0, y: 0.0} => println!("In the middle!"),
        Point{x: 0.0, y} => println!("on x axis!"),
        Point{x, y:0.0} => println!("on y axis!"),
        Point{x, y} if x > 0.0 && y > 0.0 => println!("1st quadrant"),
        Point{x, y} if x < 0.0 && y > 0.0 => println!("2nd quadrant"),
        Point{x, y} if x < 0.0 && y < 0.0 => println!("3rd quadrant"),
        Point{x, y} if x > 0.0 && y < 0.0 => println!("4th quadrant"),
        _ => ()
    }
}
