use std::collections::HashMap;
use std::fs;
use std::cmp;
// use trees::{tr, Node};

// cd x
// cd ..
// cd /
// ls

fn main() {
    let map: HashMap<String, (Vec<String>, i32)> = read_input("input.txt");
    let mut p1: i32 = 0;

    fn dfs(map: &mut &HashMap<String, (Vec<String>, i32)>, curr: String, p1: &mut i32) -> i32 {
        let (children, size) = &map.get(&curr).unwrap();

        if children.len() == 0 {
            return *size;
        }

        let mut sum: i32 = 0;

        for v in children {
            sum += dfs(map, v.to_string(), p1);
        }

        if sum <= 100000 {
            *p1 += sum;
        }

        return sum;
    }

    let used_space = dfs(&mut &map, "/".to_string(), &mut p1);

    let space_required = 30000000 + (used_space - 70000000);
    let mut p2: i32 = i32::max_value();
    println!("{}", space_required);

    fn dfsp2(map: &HashMap<String, (Vec<String>, i32)>, curr: String, p2: &mut i32, space_required: i32) -> i32 {
        let (children, size) = &map.get(&curr).unwrap();

        if children.len() == 0 {
            return *size;
        }

        let mut sum: i32 = 0;

        for v in children {
            sum += dfsp2(map, v.to_string(), p2, space_required);
        }

        if sum >= space_required {
            *p2 = cmp::min(*p2, sum);
        }

        return sum;
    }

    dfsp2(&map, "/".to_string(), &mut p2, space_required);

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn read_input(filename: &str) -> HashMap<String, (Vec<String>, i32)> {
    let str = fs::read_to_string(filename).expect("failed to read file");
    let terminal: Vec<&str> = str.lines().collect();
    let mut map: HashMap<String, (Vec<String>, i32)> = HashMap::new();
    let mut stack: Vec<&str> = vec![];
    // let mut s = String::from("");

    // let mut dirs: Vec<String> = vec![];
    fn add_node(node: &str, size: i32, map: &mut HashMap<String, (Vec<String>, i32)>) -> bool {
        match map.get(node) {
            None => {
                map.insert((*node).to_string(), (Vec::new(), size));
                true
            }
            _ => false,
        }
    }

    fn add_edge(map: &mut HashMap<String, (Vec<String>, i32)>, edge: (&str, &str, i32)) {
        add_node(edge.0, 0, map);
        add_node(edge.1, edge.2, map);

        map.entry(edge.0.to_string()).and_modify(|e| {
            e.1 = edge.2;
            e.0.push(edge.1.to_string());
        });
    }

    for x in terminal {
        if x == "$ cd .." {
            // pop from stack
            stack.pop().unwrap();
        } else if x.contains("$ cd ") {
            // figure out file name and put it on stack
            let (_, file) = x.split_once("$ cd ").expect("expected file");
            stack.push(file);
        } else {
            if x == "$ ls" {
                continue;
            }
            // add all directories to map
            let (first, second) = x.split_once(" ").expect("Weird split behaviour");
            let (name, size) = match first {
                "dir" => (second, 0),
                _ => (second, first.parse::<i32>().unwrap()),
            };

            let path = stack.join("/");
            let new_path = path.clone() + "/" + name;
            add_edge(&mut map, (path.as_str(), new_path.as_str(), size));
        }
    }

    return map;
}
