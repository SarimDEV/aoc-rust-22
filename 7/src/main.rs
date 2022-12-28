use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let terminal = read_input("input.txt");
    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut dir: Vec<String> = Vec::new();

    for cmd in terminal {
        if cmd == "$ ls" || cmd.contains("dir ") {
            continue
        }

        let split_cmd: Vec<_> = cmd.split_whitespace().collect();
        match split_cmd[..] {
            ["$", "cd", ".."] => {
                dir.pop();
            }
            ["$", "cd", name] => {
                dir.push(name.to_string());
            },
            [size, _name] => {
                let parsed_size = size.parse::<u32>().unwrap();
                
                for d in &dir {
                    *sizes.entry(d.to_string()).or_insert(0) += parsed_size;
                }
            }
            _ => {},
        }
    }

    // println!("P1: {}", p1);
    // println!("P2: {}", p2);
}

fn read_input(filename: &str) -> Vec<String> {
    let str: Vec<String> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect();
    return str;
}
