use std::fs;
use std::collections::HashSet;

fn main() {
    let str = read_input("input.txt");
    let mut set: HashSet<char> = HashSet::new();
    let mut p1: usize = 0;
    let mut l = 0;

    for (r, c) in str.chars().enumerate() {
        while set.contains(&c) {
            let char_to_remove = str.as_bytes()[l] as char;
            set.remove(&char_to_remove);
            l += 1;
        }
        set.insert(c);

        if r - l + 1 >= 4 {
            p1 = r + 1;
            break;
        }
    }

    let mut set1: HashSet<char> = HashSet::new();
    let mut p2: usize = 0;
    let mut l = 0;

    for (r, c) in str.chars().enumerate() {
        while set1.contains(&c) {
            let char_to_remove = str.as_bytes()[l] as char;
            set1.remove(&char_to_remove);
            l += 1;
        }
        set1.insert(c);

        if r - l + 1 >= 14 {
            p2 = r + 1;
            break;
        }
    }

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn read_input(filename: &str) -> String {
    let str = fs::read_to_string(filename).expect("failed to read file");
    return str;
}