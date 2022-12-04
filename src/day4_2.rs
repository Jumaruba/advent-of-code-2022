use std::io; 

fn remove_rn(s: &mut String){
    s.pop();
    s.pop();
}

fn main(){
    let mut res = 0; 
    loop { 
        let mut i = String::new(); 
        io::stdin().read_line(&mut i).unwrap(); 
        remove_rn(&mut i);

        if i == ""{
            println!("{}", res);
            break; 
        }

        let t: Vec<&str> = i.split(",").collect(); 
        let t1: Vec<&str> = t[0].split("-").collect(); 
        let t2: Vec<&str> = t[1].split("-").collect(); 

        let i1_b:i32 = t1[0].parse().unwrap(); 
        let i1_e:i32 = t1[1].parse().unwrap(); 
        let i2_b:i32 = t2[0].parse().unwrap(); 
        let i2_e:i32 = t2[1].parse().unwrap(); 


        if i1_b <= i2_b && i2_e <= i1_e {
            res += 1
        } else if i2_b <= i1_b && i1_e <= i2_e {
            res += 1;
        } else if i2_b <= i1_e && i1_e <= i2_e {
            res += 1; 
        } else if i1_b <= i2_e && i2_e <= i1_e {
            res += 1;
        }
    }
}