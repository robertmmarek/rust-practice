use std::vec;

#[derive(Debug)]
struct Student{
    id: u32,
    name: String,
    surname: String,
    age: i32,
    marks: vec::Vec<u32>
}

fn build_user(id: u32, name: String, surname: String, age: i32, marks: vec::Vec<u32>) -> Student
{
    Student{
        id,
        name,
        surname,
        age, 
        marks
    }
}

fn main() {
    let mut roll: vec::Vec<Student> = vec::Vec::new();
    let mut marks: vec::Vec<u32> = vec![2, 3, 2];
    let mut marks2 = marks[..].to_vec();
    roll.push( build_user(0, String::from("Jack"), String::from("Strong"), 20, marks ));
    roll.push( build_user(1, String::from("Jack2"), String::from("Strong2"), 20, marks2 ));

    for el in roll {
        println!("{:?}", el);
    }
}
