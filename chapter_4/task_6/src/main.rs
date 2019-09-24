use std::io;
use std::vec;
use std::cmp::Ordering;

#[derive(Eq)]
enum ElementTypes{
    CHARS,
    NUMBERS
}

impl Ord for ElementTypes{
    fn cmp(&self, other: &Self) -> Ordering{
        match (self, other){
            (ElementTypes::CHARS, ElementTypes::CHARS) => {Ordering::Equal},
            (ElementTypes::NUMBERS, ElementTypes::NUMBERS) => {Ordering::Equal},
            (ElementTypes::NUMBERS, ElementTypes::CHARS) => {Ordering::Greater},
            (ElementTypes::CHARS, ElementTypes::NUMBERS) => {Ordering::Less},
        }
    }
}

impl PartialEq for ElementTypes{
    fn eq(&self, other: &Self) -> bool{
        match (self, other){
            (ElementTypes::CHARS, ElementTypes::CHARS) => {true},
            (ElementTypes::NUMBERS, ElementTypes::NUMBERS) => {true},
            _ => {false}
        }
    }
}

impl PartialOrd for ElementTypes{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        match (self, other){
            (ElementTypes::CHARS, ElementTypes::CHARS) => {Some(Ordering::Equal)},
            (ElementTypes::NUMBERS, ElementTypes::NUMBERS) => {Some(Ordering::Equal)},
            (ElementTypes::NUMBERS, ElementTypes::CHARS) => {Some(Ordering::Greater)},
            (ElementTypes::CHARS, ElementTypes::NUMBERS) => {Some(Ordering::Less)},
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);

    let mut contain_list: vec::Vec<ElementTypes> = vec::Vec::new();

    let en1: ElementTypes = ElementTypes::CHARS;
    let en2: ElementTypes = ElementTypes::CHARS;

    for el in buf.chars()
    {
        match el{
            '0'..='9' => {contain_list.push(ElementTypes::NUMBERS);},
            'a' ..= 'z' | 'A' ..= 'Z' => {contain_list.push(ElementTypes::CHARS);},
            _ => ()
        }
        println!("{}", el);
    }

    contain_list.sort();
    contain_list.dedup();

    for el in contain_list{
        match el{
            ElementTypes::CHARS => {println!("contains chars!");},
            ElementTypes::NUMBERS => {println!("contains numbers!")}
        }
    }
}
