use std::io;
use std::collections::{HashSet, hash_set};

fn main() {
    let mut diff_chars: Vec<char> = Vec::new(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); 
    let mut chars : Vec<char> = input.chars().collect();

    for i in 0..14 {
        diff_chars.push(chars[i]); 
    }


    // If the first characters are already the begginning
    if is_unique(&diff_chars) {
        println!("{}, {:?}", 1, diff_chars);
    }

    let count = input.len(); 
    for i in 14..count {
        let c: &char = &chars[i];
        diff_chars.remove(0);
        diff_chars.push(*c);
        if is_unique(&diff_chars) { 
            println!("{} {:?}", i+1, diff_chars);
            break; 
        } 
        
    }
}

fn is_unique(v: &Vec<char>) -> bool {
    let mut h : HashSet<char> = HashSet::new();
    for i in v.into_iter() { 
        if h.contains(&i) {
            return false;
        } else {
            h.insert(*i); 
        }
    }
    true 
}

