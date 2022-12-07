use std::io;

#[derive(Debug)]
pub struct Node {
    size: u32,
    children: Vec<Node>,
    is_dir: bool,
    name: String,
}

impl Node {
    pub fn new(is_dir: bool, name: String, size: u32) -> Self {
        Self {
            size,
            children: Vec::new(),
            is_dir,
            name,
        }
    }
}

// ======================================================================
fn remove_last_n(s: &mut String, times: i32) {
    for i in 0..times {
        s.pop();
    }
}

fn is_command(s: &mut String) -> bool {
    println!("{}", s);
    if s.chars().nth(0).unwrap() == '$' {
        return true;
    }
    false
}

fn process_dir(node: &mut Node, i: String) {
    let name = i.split(" ").collect::<Vec<&str>>()[1].to_string();
    let child: Node = Node::new(true, name, 0);
    node.children.push(child);
}

fn process_file(node: &mut Node, i: String) {
    let v: Vec<&str> = i.split(" ").collect();
    let size = v[0].parse::<u32>().unwrap();
    let name = v[1].to_string();
    let child: Node = Node::new(false, name, size);
    node.children.push(child);
}

fn process_ls(node: &mut Node) -> String {
    loop {
        let mut i = String::new();
        io::stdin().read_line(&mut i).unwrap();
        if i == "" {
            return "".to_string();
        }

        remove_last_n(&mut i, 2);
        if is_command(&mut i) {
            return i;
        } else if i.chars().nth(0).unwrap() == 'd' {
            process_dir(node, i);
        } else {
            process_file(node, i);
        }
    }
}
fn process_cd(node: &mut Node, folder: String) {
    println!("FOLDER {}", folder);
    for f in node.children.iter_mut() {
        if f.name == folder {
            create_tree(f);
        }
    }
}

fn create_tree(root: &mut Node) {
    let mut changed = false;
    let mut i = "".to_string();
    loop {
        if !changed {
            i = String::new();
            io::stdin().read_line(&mut i).unwrap();
            remove_last_n(&mut i, 2);
        }
        changed = false;
        if i == "" {
            break;
        }

        if is_command(&mut i) {
            let mut args: Vec<String> = i
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string())
                .collect();
            let mut cmd: String = args[1].to_string();
            if cmd == "ls" {
                i = process_ls(root);
                changed = true;
                continue;
            }else if cmd == "cd" {
                if args[2] == ".." {
                    break;
                } else if args[2] != "/" {
                    process_cd(root, args[2].to_string());
                }
            }
        }
    }
    println!("{:?}", root);
}


fn main() {
    let mut root = Node::new(true, "/".to_string(), 0);
    create_tree(&mut root);
    //calculate_size(&mut root); 
}
