use queues::*;
use std::io;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn remove_last_n(s: &mut String, times: i32) {
    for i in 0..times {
        s.pop();
    }
}

fn get_positions(pos: &String) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for (i, c) in pos.chars().enumerate() {
        if c != ' ' {
            v.push(i);
        }
    }
    v
}

fn read_elements(q: &mut Vec<String>, v: &Vec<usize>, line: &String) {
    for (i, pos) in v.iter().enumerate() {
        let c = line.chars().nth(*pos).unwrap();
        if c != ' ' {
            q[i] = format!("{}{}", c, q[i]);
        }
    }
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    let mut vq: Vec<String>; 
    loop {
        let mut i = String::new();
        io::stdin().read_line(&mut i).unwrap();

        if i == LINE_ENDING || i == "" {
            break;
        }
        remove_last_n(&mut i, 2);   // remove \r\n
        inputs.push(i); // Add this input to the list of inputs
    }

    // Get the positions of the string that contains a letter.
    let mut letter_pos = get_positions(&inputs[inputs.len() - 1]);
    // Removes the line that contains the positions
    inputs.pop();   // remove the letter pos
    vq = vec![String::new(); letter_pos.len()];

    for line in inputs.into_iter() {
        read_elements(&mut vq, &letter_pos, &line);
    }

    loop {
        let mut i = String::new();
        io::stdin().read_line(&mut i).unwrap();

        if i == "" {
            break;
        }
        remove_last_n(&mut i, 2); 
        let positions: Vec<&str> = i.split(" ").collect();
        let times: i16 = positions[1].parse().unwrap(); 
        for _ in 0..times {
            let from: i16 = positions[3].parse().unwrap();
            let to : i16 = positions[5].parse().unwrap(); 
            let c_poped = vq[(from-1) as usize].pop().unwrap();
            vq[(to-1) as usize].push(c_poped);
        }

    }
    let mut s = String::new(); 
    for f in vq.into_iter() {
        s.push(f.chars().nth(f.len()-1).unwrap()); 
    }

    println!("{}", s);


}
