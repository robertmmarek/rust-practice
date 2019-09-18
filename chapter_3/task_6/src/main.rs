
mod books{
    static mut COUNTER: u32 = 0;

    #[derive(Debug)]
    pub struct Book{
        id: u32,
        author: String,
        title: String,
        nb_copies: u32,
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

        fn issue_book(&mut self) -> Result<u32, &'static str>{
            if self.nb_copies == 0 {
                Err("No more books left!")
            }else{
                self.nb_copies -= 1;
                Ok(self.nb_copies)
            }
        }

        fn add_book(&mut self) -> Result<u32, &'static str>{
            self.nb_copies += 1;
            Ok(self.nb_copies)
        }
    }

    
}

use std::io;
use std::vec::Vec;

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
            Ok(n) => {n},
            Err(e) => {panic!("wrong input!")}
        };
        let mut selection: u32 = match input.parse(){
            Ok(n) => { if (1..6).contains(&n) {n} else {1}},
            Err(e) => {1}
        };
        
    }
}
