struct Student{
    name: String,
    age: u32,
    roll_number: u32
}

impl Drop for Student{
    fn drop(&mut self){
        let junior_senior = match self.age{
            x if x >= 18 => { String::from("senior") },
            x if x < 18 => { String::from("junior") },
            _ => { String::from("") }
        };
        println!("Roll number {} has name {} with age {} and is a {}", 
                 self.roll_number,
                 self.name,
                 self.age,
                 junior_senior);
        
    }
}

impl std::fmt::Debug for Student{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "({:?}, {:?}, {:?})", self.name, self.age, self.roll_number)
    }
}

fn main() {
    {
        let test_student = Student { name: String::from("Test"), age: 20, roll_number: 4 };
        println!("{:?}", test_student);
    }
    
}
