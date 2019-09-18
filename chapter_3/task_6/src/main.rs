
mod books{
    static mut COUNTER: u32 = 0;

    #[derive(Debug)]
    pub struct Book{
        pub id: u32,
        pub author: String,
        pub title: String,
        pub nb_copies: u32,
        _priv: ()
    }

    impl Book{
        pub fn new(author: String, title: String) -> Book{
            unsafe{
            let ret = Book { id: COUNTER,
                             author: author,
                             title: title,
                             nb_copies: 1,
                             _priv: () };
            
                COUNTER += 1;
            
            return ret
            }
        }

        pub fn issue_book(&mut self) -> Result<u32, &'static str>{
            if self.nb_copies == 0 {
                Err("No more books left!")
            }else{
                self.nb_copies -= 1;
                Ok(self.nb_copies)
            }
        }

        pub fn add_book(&mut self) -> Result<u32, &'static str>{
            self.nb_copies += 1;
            Ok(self.nb_copies)
        }
    }

    
}

use std::io;
use std::iter;
use std::vec::Vec;

fn display_summary(lib: &Vec<books::Book>){
    for book in lib.iter()
    {
        println!("{:?}", book);
    }
}

fn display_author(lib: &Vec<books::Book>)
{
    let mut author = String::new();
    println!("give author");
    io::stdin().read_line(&mut author);
    author = String::from(author.trim());

    for book in lib.iter()
    {
        if book.author == author {
            println!("{:?}", book);
        }
    }
}

fn display_total_books(lib: &Vec<books::Book>)
{
    let mut sum = 0;
    for el in lib.iter(){
        sum += el.nb_copies;
    }
    println!("Total books in library: {}", sum);
}

fn display_title(lib: &Vec<books::Book>)
{
    let mut title = String::new();
    println!("give title");
    io::stdin().read_line(&mut title);
    title = String::from(title.trim());

    for book in lib.iter()
    {
        if book.title == title {
            println!("{:?}", book);
        }
    }
}

fn issue_book(lib: &mut Vec<books::Book>)
{
    let mut author = String::new();
    println!("give author");
    io::stdin().read_line(&mut author);
    author = String::from(author.trim());

    let mut title = String::new();
    println!("give title");
    io::stdin().read_line(&mut title).expect("error reading title");
    title = String::from(title.trim());

    match lib.iter_mut().find(|x| x.author==author && x.title==title){
        Some(el) => {
            match el.issue_book(){
                Ok(n) => {println!("Issued a book {:?}, left: {}", el, n);},
                Err(err) => {println!("No more books of this title in library!");}
            }
        },
        None => {println!("This book does not exist!");}
    }
}

fn add_new_book(lib: &mut Vec<books::Book>)
{
    let mut author_input = String::new();
    let mut title_input = String::new();
    println!("input author");
    io::stdin().read_line(&mut author_input).expect("error reading author");
    author_input = String::from(author_input.trim());
    println!("input title");
    io::stdin().read_line(&mut title_input).expect("error reading title");
    title_input = String::from(title_input.trim());

    match lib.iter_mut().find(|el| el.author == author_input && el.title == title_input){
        Some(r) => {
            r.add_book();
            println!("Added book to existing position")
        },
        None => {
            lib.push(books::Book::new(author_input, title_input));
            println!("Added new book!");
        }
    }

}

fn main() {
    let mut library: Vec<books::Book> = Vec::new();

    loop {
        println!("Select: ");
        println!("1 - display book information");
        println!("2 - add new book");
        println!("3 - display books of author");
        println!("4 - display books of specific title");
        println!("5 - total books in library");
        println!("6 - issue a book");
        let mut input = String::new();
        match io::stdin().read_line(&mut input){
            Ok(n) => {input = String::from(input.trim());},
            Err(e) => {panic!("wrong input!")}
        };
        let mut selection: u32 = match input.parse(){
            Ok(n) => { if (1..7).contains(&n) {n} else {1}},
            Err(e) => {1}
        };

        match selection {
            1 => display_summary(&library),
            2 => add_new_book(&mut library),
            3 => display_author(&library),
            4 => display_title(&library),
            5 => display_total_books(&library),
            6 => issue_book(&mut library),
            _ => panic!("wrong option selected!")
        }
        
    }
}
