use std::fs;

fn main() {
    let (map, instructions): (Vec<Vec<char>>, Vec<Vec<u32>>) = read_input("input.txt");
    // let mut flippedMatrix: Vec<Vec<char>> = vec![vec![]];
    let m = map.len();
    let n = map[0].len();
    let mut rot_mat: Vec<Vec<char>> = vec![vec![]; n];

    // Rotate matrix 
    for i in (0..m).rev() {
        for j in 0..=n-1 {
            if map[i][j] == '.' {
                continue;
            }
            rot_mat[j].push(map[i][j])
        }
    }

    let mut rot_mat_copy: Vec<Vec<char>> = rot_mat.clone();

    for instruction in &instructions {
        let (amt, from, to) = (instruction[0], instruction[1], instruction[2]);

        for _i in 0..amt {
            let val = rot_mat[from as usize - 1].pop().unwrap();
            rot_mat[to as usize - 1].push(val);
        }
    }

    let mut p1 = String::from("");

    for i in 0..n {
        let last_val = rot_mat[i].pop().unwrap();
        p1.push(last_val);
    }

    for instruction in &instructions {
        let (amt, from, to) = (instruction[0], instruction[1], instruction[2]);

        let mut stack: Vec<char> = vec![];

        for _i in 0..amt {
            let val = rot_mat_copy[from as usize - 1].pop().unwrap();
            stack.push(val)
        }

        for _i in 0..amt {
            rot_mat_copy[to as usize - 1].push(stack.pop().unwrap());
        }
    }

    let mut p2: String = String::from("");

    for i in 0..n {
        let last_val = rot_mat_copy[i].pop().unwrap();
        p2.push(last_val);
    }

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<Vec<u32>>) {
    let str = fs::read_to_string(filename).expect("failed to read file");
    let (map, instructions) = str.split_once("\n\n").expect("Failed to parse");
    let mut map: Vec<Vec<char>> = map
        .lines()
        .map(|l| l.replace("    ", "[.]").replace(" ", ""))
        .map(|l| l.chars().filter(|x| *x != '[' && *x != ']').collect())
        .collect();
    map.pop();

    let instructions: Vec<Vec<u32>> = instructions
        .lines()
        .map(|l| {
            l.replace("move ", "")
                .replace(" from ", ",")
                .replace(" to ", ",")
        })
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<u32>().expect("Failed to parse"))
                .collect()
        })
        .collect();

    return (map, instructions);
}
