use std::io; 
use std::io::Read; 


fn main(){
    let mut total: u32 = 0; 
    let mut counter = 0; 
    loop {
        counter += 1; 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); 
        input.pop(); 
        input.pop();
        let vec: Vec<&str> = input.split(" ").collect(); 
        if vec[1] == "X"{
            total += 1; 
        } else if vec[1] == "Y" {
            total += 2; 
        } else if vec[1] == "Z" {
            total += 3; 
        }

        if (vec[0] == "A" && vec[1] == "X") || (vec[0] == "B" && vec[1] == "Y") || (vec[0] == "C" && vec[1] == "Z") {
            total += 3
        } else if (vec[0] == "A" && vec[1] == "Y") || (vec[0] == "B" && vec[1] == "Z") || (vec[0] == "C" && vec[1] == "X"){
            total += 6
        } 
        println!("{} : {}",counter, total);
    } 
}