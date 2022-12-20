// use std::cmp;
use std::fs;

fn main() {
    // --snip--
    let input: Vec<Vec<String>> = read_input("input.txt");

    println!("{:?}", input);
}

fn read_input(filename: &str) -> Vec<Vec<String>> {
    let test: Vec<Vec<String>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| {
            let (first, last) = l.split_at(l.len() / 2);
            vec![first.to_string(), last.to_string()]
        })
        .collect();

    return test;
}
