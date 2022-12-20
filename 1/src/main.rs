use std::cmp;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "input.txt");

    let input: Vec<Vec<u32>> = read_input("input.txt");
    let mut p1: u32 = 0;

    for i in &input {
        let mut local_max: u32 = 0;
        for cal in i {
            local_max = local_max + *cal
        }
        p1 = cmp::max(p1, local_max)
    }

    let mut calories = vec![];

    for i in &input {
        let mut local_max: u32 = 0;
        for cal in i {
            local_max = local_max + *cal;
        }
        calories.push(local_max);
    }

    calories.sort_by(|a,b| b.cmp(a));

    let p2 = calories[0] + calories[1] + calories[2];

    println!("P1: {p1}");
    println!("P1: {p2}");
}

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let test: Vec<Vec<u32>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .split("\n\n")
        .map(|l| l.lines().map(|l| l.parse::<u32>().unwrap()).collect())
        .collect();

    return test;
}