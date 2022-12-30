use std::cmp::Ordering;
use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::vec;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    fcost: usize,
    hcost: usize,
    pos: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .fcost
            .cmp(&self.fcost)
            .then_with(|| self.hcost.cmp(&other.hcost))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn h(pos: (usize, usize), end: (usize, usize)) -> usize {
    let (cr, cc) = pos;
    let (er, ec) = end;

    return cr.abs_diff(er).pow(2) + cc.abs_diff(ec).pow(2);
}

fn a_star(start: (usize, usize), end: (usize, usize), map: &Vec<Vec<char>>) -> i32 {
    let n = map.len() as i32;
    let m = map[0].len() as i32;
    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, 1, -1];
    let mut shortest_g = i32::max_value();
    let mut min_heap = BinaryHeap::new();

    let mut g_scores: HashMap<(usize, usize), usize> = HashMap::new();
    g_scores.insert(start, 0);
    let mut f_scores: HashMap<(usize, usize), usize> = HashMap::new();
    f_scores.insert(start, h(start, end));

    // g, h, f
    min_heap.push(State {
        fcost: h(start, end),
        hcost: h(start, end),
        pos: start,
    });
    while min_heap.len() != 0 {
        let val = min_heap.pop().unwrap();
        let pos = val.pos;
        // back += 1;
        if pos == end {
            shortest_g = *g_scores.get(&pos).unwrap() as i32;
            break;
        }
        // visited.insert(pos, true);

        for i in 0..4 {
            let rr = pos.0 as i32 + dr[i];
            let cc = pos.1 as i32 + dc[i];

            let child_cell = (rr as usize, cc as usize);

            if rr < 0 || cc < 0 {
                continue;
            };
            if rr >= n || cc >= m {
                continue;
            };

            let curr = map[pos.0][pos.1] as u32;

            // if curr position is greater than or equal to next pos + 1
            if curr < map[rr as usize][cc as usize] as u32 - 1 {
                continue;
            }

            let new_gscore = g_scores.get(&pos).unwrap() + 1;
            let new_fscore = new_gscore + h(child_cell, end);

            if new_fscore < *f_scores.get(&child_cell).unwrap_or(&usize::max_value()) {
                g_scores.insert(child_cell, new_gscore);
                f_scores.insert(child_cell, new_fscore);
                min_heap.push(State {
                    fcost: new_fscore,
                    hcost: h(child_cell, end),
                    pos: child_cell,
                })
            }
        }
    }

    return shortest_g;
}

fn main() {
    let mut map = read_input("input.txt");
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'S' {
                map[r][c] = 'a';
                start = (r, c);
            } else if map[r][c] == 'E' {
                map[r][c] = 'z';
                end = (r, c);
            }
        }
    }

    let p1 = a_star(start, end, &map);
    let p2 = {
        let mut s_pos = vec![];
        let mut res = usize::max_value();
        for r in 0..map.len() {
            for c in 0..map[0].len() {
                if map[r][c] == 'a' {
                    s_pos.push((r,c));
                }
            }
        }

        for pos in s_pos {
            res = cmp::min(a_star(pos, end, &map) as usize, res);
        }

        res
    };

    println!("P1 {}", p1);
    println!("P2 {}", p2);
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    return input;
}
