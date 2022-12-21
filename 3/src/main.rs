// use std::cmp;
use std::fs;

fn main() {
    let mut input: Vec<String> = vec![];
    read_input("input.txt", &mut input);
    let mut p1 = 0;

    for i in &input {
        let mut arr: [u32; 52] = [0; 52];
        let (first, second) = i.split_at(i.len() / 2);

        for c in first.chars() {
            let int = convert_char_num(c);
            arr[int as usize] = 1;
        }

        for c in second.chars() {
            let int = convert_char_num(c);
            if arr[int as usize] == 1 {
                p1 += int + 1;
                break;
            }
        }
    }

    let mut p2: u32 = 0;
    let len_input = len_vec(&mut input);
    let iter = 0..len_input;

    for n in (iter).step_by(3) {
        let mut arr: [u32; 52] = [0; 52];
        let (first, second, third) = (&input[n], &input[n + 1], &input[n + 2]);
        for c in first.chars() {
            let int = convert_char_num(c);
            arr[int as usize] = 1;
        }
        for c in second.chars() {
            let int = convert_char_num(c);
            if arr[int as usize] == 1 {
                arr[int as usize] = 2
            }
        }
        for c in third.chars() {
            let int = convert_char_num(c);
            if arr[int as usize] == 2 {
                p2 += int + 1;
                break;
            }
        }
    }

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn len_vec(vec: &mut Vec<String>) -> usize {
    return vec.len();
}

fn convert_char_num(c: char) -> u32 {
    let int = c as u32;

    if int < 97 {
        return int - 39;
    }

    return int - 97;
}

fn read_input(filename: &str, input: &mut Vec<String>) {
    let test: Vec<String> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect();

    for i in test {
        input.push(i)
    }
}
