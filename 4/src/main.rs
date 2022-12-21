// use std::cmp;
use std::fs;
// [[1,2], [2,3]]

fn main() {
    let mut input: Vec<Vec<Vec<u32>>> = vec![];
    read_input("input.txt", &mut input);
    let mut p1 = 0;

    for i in &input {
        let (f_start, f_end, s_start, s_end) = (i[0][0], i[0][1], i[1][0], i[1][1]);
        if s_end <= f_end && s_start >= f_start {
            p1 += 1;
        } else if f_end <= s_end && f_start >= s_start {
            p1 += 1;
        }
    }

    let mut p2 = 0;

    for i in &input {
        let (f_start, f_end, s_start, s_end) = (i[0][0], i[0][1], i[1][0], i[1][1]);
        if s_start <= f_end && s_end >= f_start {
            p2 += 1;
        }
    }

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn read_input(filename: &str, input: &mut Vec<Vec<Vec<u32>>>) {
    let test: Vec<Vec<Vec<u32>>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| {
            l.split(",").map(|f| {
                f.split('-').map(|i| {
                    i.parse::<u32>().unwrap()
                }).collect()
            }).collect()
        }).collect();

    for i in test {
        input.push(i)
    }
}
