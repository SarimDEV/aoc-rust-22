// use std::cmp;
use std::fs;

fn main() {
    // --snip--
    let input: Vec<Vec<char>> = read_input("input.txt");
    let mut p1: u32 = 0;

    for i in &input {
        let opponent = i[0];
        let you = i[1];

        p1 += outcome_score(opponent, you) + shape_score(you);
    }

    let mut p2: u32 = 0;

    for i in &input {
        let opponent = i[0];
        let you = i[1];

        p2 += score(opponent, you)
    }

    println!("P1: {p1}");
    println!("P2: {p2}");
}

fn shape_score(you: char) -> u32 {
    if you == 'X' {
        return 1;
    } else if you == 'Y' {
        return 2;
    } else {
        return 3;
    };
}

// A , B , C
// X , Y , Z
// lose, draw, win

fn to_win(opponent: char) -> char {
    if opponent == 'A' {
        return 'Y';
    } else if opponent == 'B' {
        return 'Z';
    } else {
        return 'X';
    }
}

fn to_draw(opponent: char) -> char {
    if opponent == 'A' {
        return 'X';
    } else if opponent == 'B' {
        return 'Y';
    } else {
        return 'Z';
    }
}

fn to_lose(opponent: char) -> char {
    if opponent == 'A' {
        return 'Z';
    } else if opponent == 'B' {
        return 'X';
    } else {
        return 'Y';
    }
}

fn score(opponent: char, you: char) -> u32 {
    if you == 'X' {
        let shape_score = shape_score(to_lose(opponent));
        return 0 + shape_score;
    }
    else if you == 'Y' {
        let shape_score = shape_score(to_draw(opponent));
        return 3 + shape_score;
    }
    else {
        let shape_score = shape_score(to_win(opponent));
        return 6 + shape_score;
    }
}

fn outcome_score(opponent: char, you: char) -> u32 {
    if (opponent == 'A' && you == 'X')
        || (opponent == 'B' && you == 'Y')
        || (opponent == 'C' && you == 'Z')
    {
        return 3;
    }

    if (opponent == 'A' && you == 'Y')
        || (opponent == 'B' && you == 'Z')
        || (opponent == 'C' && you == 'X')
    {
        println!("{opponent}, {you}");
        return 6;
    }

    return 0;
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let test: Vec<Vec<char>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.split(' ').map(|v| v.parse::<char>().unwrap()).collect())
        .collect();

    return test;
}
