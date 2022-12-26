use std::{cmp, fs};

fn main() {
    let map = read_input("input.txt");
    let mut p1: u32 = 0;
    let m = map.len();
    let n = map[0].len();

    // worst solution I could think of. Prob optimizations or DP way to do this.
    // brain is not working right now.
    for (i, row) in map.iter().enumerate() {
        for (j, val) in map[i].iter().enumerate() {
            if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                p1 += 1;
                continue;
            } else {
                let mut max_r = 0;
                let mut max_l = 0;
                let mut max_d = 0;
                let mut max_u = 0;

                for r in j + 1..n {
                    max_r = cmp::max(max_r, row[r])
                }
                for l in 0..j {
                    max_l = cmp::max(max_l, row[l])
                }
                for d in i + 1..m {
                    max_d = cmp::max(max_d, map[d][j])
                }
                for u in 0..i {
                    max_u = cmp::max(max_u, map[u][j])
                }

                let min = cmp::min(cmp::min(max_r, max_l), cmp::min(max_u, max_d));

                if min < *val {
                    p1 += 1
                }
            }
        }
    }

    let mut p2 = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, val) in map[i].iter().enumerate() {
            if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                continue;
            } else {
                let mut view_r = 0;
                let mut view_l = 0;
                let mut view_d = 0;
                let mut view_u = 0;

                for r in j + 1..n {
                    view_r += 1;
                    if row[r] >= *val {
                        break;
                    }
                }
                for l in (0..j).rev() {
                    view_l += 1;
                    if row[l] >= *val {
                        break;
                    }
                }
                for d in i + 1..m {
                    view_d += 1;
                    if map[d][j] >= *val {
                        break;
                    }
                }
                for u in (0..i).rev() {
                    view_u += 1;
                    if map[u][j] >= *val {
                        break;
                    }
                }

                let scenic_score = view_u * view_d * view_l * view_r;
                println!("{} {} {} {} {} {}", view_u, view_d, view_l, view_r, i, j);

                p2 = cmp::max(p2, scenic_score);
            }
        }
    }

    println!("P1: {}", p1);
    println!("P2: {}", p2);
}

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    let map: Vec<Vec<u8>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect();
    return map;
}
