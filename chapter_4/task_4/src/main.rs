use std::vec;

#[derive(Debug)]
enum Grade{
    One,
    Two,
    Three,
    Four,
    Five,
}

struct Student{
    grades: vec::Vec<f32>
}

impl Student{
    pub fn mean(&self) -> f32{
        let mut ret = 0.0;

        for el in self.grades.iter(){
            ret += el;
        }

        if self.grades.len() > 0{
            ret = ret / (self.grades.len() as f32);
        }
        ret
    }

    pub fn add_grade(&mut self, grade: f32){
        self.grades.push(grade);
    }
}

fn main() {
    let mut student = Student { grades: vec![2., 3., 4.]};

    let final_grade = match Some(student.mean()) {
        Some(x) if x < 1.5 => Grade::One,
        Some(x) if x < 2.5 => Grade::Two,
        Some(x) if x < 3.5 => Grade::Three,
        Some(x) if x < 4.5 => Grade::Four,
        Some(x) if x < 5.5 => Grade::Five,
        _ => Grade::One
    };

    println!("{:?}", final_grade);
}
