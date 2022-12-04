use std::collections::HashSet;
use std::io::{self};
use std::cmp::max; 
fn get_priority(c: &char) -> u8 {
    if *c >= 'a' && *c <= 'z' {
        return (*c as u8) - b'a' + 1;
    }
    return (*c as u8) - b'A' + 27;
}

fn main() {
    let mut total: i32 = 0;
    loop {
        let mut m = 0;
        let mut hash: HashSet<char> = HashSet::new();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        if input == "" {
            println!("{}", total); 
            break; 
        }

        let size = input.len() / 2;
        for i in 0..size-1 {
            hash.insert(input.chars().nth(i).unwrap());
        }

        for i in size-1..size*2-1 {
            let c = &input.chars().nth(i).unwrap();
            if hash.contains(&c) {
                m = max(get_priority(c), m);
            }
        }
        total += m as i32; 

    }
}
