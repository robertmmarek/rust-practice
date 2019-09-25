use regex::Regex;

#[derive(Debug)]
struct DateInfo{
    day: i32,
    month: i32,
    year: i32
}

fn main() {
    let in_string = "It was on 2019-03-14, almost after a year from 2018-02-11";

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    for capt in re.captures_iter(in_string)
    {
        let day: i32 = capt.get(3).unwrap().as_str().parse().unwrap();
        let month: i32 = capt.get(2).unwrap().as_str().parse().unwrap();
        let year: i32 = capt.get(1).unwrap().as_str().parse().unwrap();

        println!("{:?}", DateInfo{day: day, month: month, year: year});
    }

}
