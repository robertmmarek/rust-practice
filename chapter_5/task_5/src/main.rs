struct GeometricSeries{
    first_number: i32,
    current_number: i32,
    ratio: i32
}

impl GeometricSeries{
    fn new(first_number: i32, ratio: i32) -> GeometricSeries{
        GeometricSeries {first_number: first_number, 
                         current_number: first_number, 
                         ratio: ratio}
    }
}

impl Iterator for GeometricSeries{
    type Item = [i32; 11];

    fn next(&mut self) -> Option<[i32; 11]>{
        let mut ret: [i32; 11] = [0; 11];
        for i in 0..11{
            ret[i] = self.current_number;
            self.current_number = self.current_number*self.ratio;
        }
        Some(ret)
    }
}

fn main() {
    let mut geometric = GeometricSeries::new(2, 2);
    println!("{:?}", geometric.next());
    println!("{:?}", geometric.next());
}
