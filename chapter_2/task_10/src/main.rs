use std::vec;

fn check_prime(number: u32) -> bool{
    let mut div_count = 0;
    let mut curr_divider = number-1;

    while curr_divider > 1 {
        div_count = match number%curr_divider==0{
            true => return false,
            false => 0,
        };
        curr_divider -= 1;
    }

    div_count == 0
}
    

fn find_primes(n_first: u32) -> vec::Vec<u32>{
    let mut ret: vec::Vec<u32> = vec::Vec::new();
    
    let mut last_number = 1;
    for i in 0..n_first
    {
        while !check_prime(last_number){last_number += 1;}
        ret.push(last_number);
        last_number += 1;
    }

    ret
}

fn main() {
    let vector_to_print = find_primes(5);

    for el in vector_to_print
    {
        print!("{} ", el);
    }
}
