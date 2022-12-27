use std::collections::HashMap;
use std::fs;

fn is_neighbour(a: (i32, i32), b: (i32, i32)) -> bool {
    let (i, j) = a;
    let (x, y) = b;

    return i32::abs(i - x) <= 1 && i32::abs(j - y) <= 1;
}

#[cfg(test)]
mod tests {
    use crate::is_neighbour;

    #[test]
    fn it_works() {
        let result = is_neighbour((0, 0), (1, 1));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (0, 1));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (1, 0));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (-1, -1));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (-1, 1));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (1, -1));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (0, -1));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (-1, 0));
        assert_eq!(result, true);
        let result = is_neighbour((0, 0), (-2, -1));
        assert_eq!(result, false);
        let result = is_neighbour((0, 0), (0, -4));
        assert_eq!(result, false);
        let result = is_neighbour((4, 2), (3, 0));
        assert_eq!(result, false);
    }
}

fn main() {
    let instructions = read_input("input.txt");
    let (mut hi, mut hj) = (0, 0);
    let (mut ti, mut tj) = (0, 0);
    let mut positons: HashMap<(i32, i32), bool> = HashMap::new();
    let mut p1 = 0;

    let mut arr: Vec<(i32, i32)> = vec![];

    for (direction, count) in &instructions {
        let step = if *direction == 'R' || *direction == 'U' {
            1
        } else {
            -1
        };

        for _i in 0..*count {
            if *direction == 'R' || *direction == 'L' {
                hi += step;
            } else {
                hj += step;
            }

            arr.push((hi, hj));
        }
    }

    for (x, y) in &arr {
        let pos = positons.get(&(ti, tj));
        match pos {
            Some(_) => {}
            None => {
                positons.insert((ti, tj), true);
                p1 += 1;
            }
        }
        // dont move if it's already a neighbour
        if is_neighbour((*x, *y), (ti, tj)) {
            continue;
        }

        // corner case
        if ti != *x && tj != *y {
            ti += if ti < *x { 1 } else { -1 };
            tj += if tj < *y { 1 } else { -1 };
            continue;
        }

        // regular case
        if tj == *y {
            ti += if ti < *x { 1 } else { -1 };
        } else {
            tj += if tj < *y { 1 } else { -1 };
        }
    }

    let mut new_pos: Vec<Vec<(i32, i32)>> = vec![vec![]; 10];
    new_pos[0] = arr;

    for j in 1..10 {
        let mut new_vec: Vec<(i32, i32)> = vec![];
        let (mut cur_x, mut cur_y) = (0, 0);
        for (x, y) in &new_pos[j-1] {

            // dont move if it's already a neighbour
            if is_neighbour((*x, *y), (cur_x, cur_y)) {
                new_vec.push((cur_x,cur_y));
                continue;
            }


            // corner case
            if cur_x != *x && cur_y != *y {
                cur_x += if cur_x < *x { 1 } else { -1 };
                cur_y += if cur_y < *y { 1 } else { -1 };
                new_vec.push((cur_x,cur_y));
                continue;
            }

            // regular case
            if cur_y == *y {
                cur_x += if cur_x < *x { 1 } else { -1 };
            } else {
                cur_y += if cur_y < *y { 1 } else { -1 };
            }
            new_vec.push((cur_x, cur_y));
        }

        new_pos[j] = new_vec;
    }

    let mut p2 = 0;
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    for position in &new_pos[9] {
        let exists = map.get(position);
        match exists {
            Some(_) => {}
            None => {
                map.insert(*position, true);
                p2 += 1;
            }
        }
    }

    println!("P1 {}", p1);
    println!("P2 {}", p2);
}

fn read_input(filename: &str) -> Vec<(char, u32)> {
    let instructions: Vec<(char, u32)> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| {
            let p = l.split_once(" ").expect("unable to split");
            let char = p.0.chars().next().unwrap();
            let value = p.1.parse::<u32>().unwrap();
            return (char, value);
        })
        .collect();
    return instructions;
}
