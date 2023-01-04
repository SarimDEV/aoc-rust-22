use std::{fs, str::Chars, cmp::Ordering};

fn main() {
    let map = read_input("input.txt");
    let mut p1 = 0;

    for (i, (left, right)) in map.iter().enumerate() {
        let res = left.compare(right);
        if res == Ordering::Less {
            p1 += i + 1;
        }
    }

    let mut single_list: Vec<&Val> = vec![];

    for (_, (left, right)) in map.iter().enumerate() {
        single_list.push(&left);
        single_list.push(&right);
    }


    let n1 = Val::List(vec![Val::List(vec![Val::Num(2)])]);
    let n2 = Val::List(vec![Val::List(vec![Val::Num(6)])]);
    single_list.push(&n1);
    single_list.push(&n2);

    single_list.sort_by(|a, b| a.compare(b));

    let mut p2 = 1;
    for (i, el) in single_list.iter().enumerate() {
        if el.compare(&n1) == Ordering::Equal || el.compare(&n2) == Ordering::Equal {
            p2 = p2 * (i + 1);
        }
    }

    println!("P1 {}", p1);
    println!("P2 {}", p2);
}

fn read_input(filename: &str) -> Vec<(Val, Val)> {
    let input: Vec<(Val, Val)> = fs::read_to_string(filename)
        .expect("failed to read file")
        .split("\n\n")
        .map(|l| {
            let (left, right) = l.split_once("\n").unwrap();
            return (Val::parse(left), Val::parse(right));
        })
        .collect();

    return input;
}

#[derive(Debug)]
enum Val {
    Num(i32),
    List(Vec<Val>),
}

impl Val {
    fn parse(s: &str) -> Val {
        let mut chars = s.chars();
        chars.next().unwrap();
        return Self::parse_into(&mut chars);
        // return Val::List(vec![Val::Num(1)]);
    }

    fn parse_into(c: &mut Chars) -> Val {
        let mut result = vec![];
        let mut curr_num = -1;

        while let Some(ch) = c.next() {
            match ch {
                '0'..='9' => {
                    if curr_num == -1 {
                        curr_num = (ch as u8 - b'0') as i32;
                    } else {
                        curr_num = (curr_num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                ',' => {
                    if curr_num > -1 {
                        result.push(Self::Num(curr_num));
                        curr_num = -1;
                    }
                }
                '[' => {
                    result.push(Self::parse_into(c));
                }
                ']' => {
                    if curr_num > -1 {
                        result.push(Self::Num(curr_num));
                    }
                    return Self::List(result);
                }
                _ => panic!("Test"),
            }
        }

        return Self::List(result);
    }

    // true if left and false if right
    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut i = 0;
                loop {
                    if left.len() <= i || right.len() <= i {
                        if left.len() < right.len() {
                            return Ordering::Less
                        } else if left.len() > right.len() {
                            return Ordering::Greater
                        } else {
                            return Ordering::Equal
                        }
                    }

                    match (&left[i], &right[i]) {
                        (Val::Num(l), Val::Num(r)) => {
                            if l < r {
                                return Ordering::Less
                            } else if l > r {
                                return Ordering::Greater
                            }
                        }
                        (Val::List(_), Val::Num(r)) => {
                            let res = left[i].compare(&Val::List(vec![Val::Num(*r)]));
                            if res != Ordering::Equal {
                                return res;
                            }
                        }
                        (Val::Num(l), Val::List(_)) => {
                            let res = Val::List(vec![Val::Num(*l)]).compare(&right[i]);
                            if res != Ordering::Equal {
                                return res;
                            }
                        }
                        (Val::List(_), Val::List(_)) => {
                            let res = left[i].compare(&right[i]);
                            if res != Ordering::Equal {
                                return res;
                            }
                        }
                    }

                    i += 1;
                }
            }
            _ => panic!("not working"),
        }
    }
}
