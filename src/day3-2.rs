use std::cmp::max;
use std::collections::HashSet;
use std::io::{self};
fn get_priority(c: &char) -> i32 {
    if *c >= 'a' && *c <= 'z' {
        return (*c as i32) - ('a' as i32) + 1;
    }
    return (*c as i32) - ('A' as i32) + 27;
}

fn main() {
    let mut total: i32 = 0;
    let mut counter = 0;
    let mut m: i32 = 0;
    let mut hash_1: HashSet<char> = HashSet::new();
    let mut hash_2: HashSet<char> = HashSet::new();

    loop {

        let mut input = String::new();
        // Reset variables
        if counter % 3 == 0 {
            m = 0;
            hash_1 = HashSet::new();
            hash_2 = HashSet::new();
        }
        io::stdin().read_line(&mut input).unwrap();
        
        if input == "" {
            println!("{}", total);
            break;
        }

        if counter % 3 == 0 {
            for c in input.chars() {
                hash_1.insert(c);
            }
        } else if counter % 3 == 1 {
            for c in input.chars() {
                hash_2.insert(c);
            }
        } else {
            for c in input.chars() {
                if hash_1.contains(&c) && hash_2.contains(&c) {
                    m = max(get_priority(&c) as i32, m);
                }
            }
            total += m as i32;
        }
        counter+= 1; 
    }
}
