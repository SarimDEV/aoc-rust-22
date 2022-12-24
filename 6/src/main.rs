use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let str = read_input("input.txt");
    let mut set: HashSet<char> = HashSet::new();
    let mut map: HashMap<char, u32> = HashMap::new();
    let mut p1: usize = 0;

    for (i, c) in str.chars().enumerate() {
        if set.len() == 4 {
            p1 = i;
            break;
        }
        set.insert(c);
        map.insert(c, map.get(&c).unwrap_or(&0)+1);
        if i < 4 {
            continue;
        }
        let char_to_remove = str.as_bytes()[i-4] as char;
        map.insert(char_to_remove, map.get(&char_to_remove).unwrap()-1);
        if map.get(&char_to_remove).unwrap() == &0 {
            set.remove(&char_to_remove);
        }
    }

    let mut set1: HashSet<char> = HashSet::new();
    let mut map1: HashMap<char, u32> = HashMap::new();
    let mut p2: usize = 0;

    for (i, c) in str.chars().enumerate() {
        if set1.len() == 14 {
            p2 = i;
            break;
        }
        set1.insert(c);
        map1.insert(c, map1.get(&c).unwrap_or(&0)+1);
        if i < 14 {
            continue;
        }
        let char_to_remove = str.as_bytes()[i-14] as char;
        map1.insert(char_to_remove, map1.get(&char_to_remove).unwrap()-1);
        if map1.get(&char_to_remove).unwrap() == &0 {
            set1.remove(&char_to_remove);
        }
    }

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn read_input(filename: &str) -> String {
    let str = fs::read_to_string(filename).expect("failed to read file");
    return str;
}
