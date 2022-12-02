use std::{cmp::max, io};

fn main() {
    let mut cal: u32 = 0;
    let mut max_cal: u32 = 0;
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input);
        if input == "\r\n" {
            max_cal = max(cal, max_cal);
            cal = 0;
        } else {
            input.pop();
            input.pop();
            cal += input.parse::<u32>().unwrap();
        }
        println!("{}", max_cal);
    }
}
