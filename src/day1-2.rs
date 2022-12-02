use std::{cmp::max, io};

fn main() {
    let mut cal: u32 = 0;
    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input);
        if input == "\r\n" || input == ""{
            if cal > max1 {
                max3 = max2;
                max2 = max1;
                max1 = cal;
            } else if cal > max2 {
                max3 = max2;
                max2 = cal;
            } else if cal > max3 {
                max3 = cal;
            }
            cal = 0;
        } 
        else {
            input.pop();
            input.pop();
            cal += input.parse::<u32>().unwrap();
        }
        
        // Handles EOF
        if input == "" {
            println!("{}", max1+max2+max3);
            break;
        }
    }

}
