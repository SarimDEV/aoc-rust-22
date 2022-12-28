use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut cycles = 1;
    let mut x = 1;
    let mut p1 = 0;

    fn calc_strength(cycles: i32, x: i32) -> i32 {
        if (cycles == 20 || (cycles - 20) % 40 == 0) && cycles < 221 {
            return x * cycles;
        }

        return 0;
    }

    for inst in &input {
        let split_inst: Vec<&str> = inst.split(" ").collect();

        match split_inst[..] {
            ["noop"] => {
                p1 += calc_strength(cycles, x);
                cycles += 1;
            }
            ["addx", val] => {
                for _i in 0..2 {
                    p1 += calc_strength(cycles, x);
                    cycles += 1;
                }
                let val = val.parse::<i32>().unwrap();
                x += val;
            }
            _ => {}
        }
    }

    let mut p2: [[char; 40]; 6] = [[' '; 40]; 6];
    let mut cycles2: i32 = 1;
    let mut x2: i32 = 1;

    fn draw(cycles: i32, x: i32, p2: &mut [[char; 40]; 6]) {
        let row = cycles / 40;
        let curr_drawn = cycles % 40 - 1;
        println!("{} {}", row, curr_drawn);

        if x == curr_drawn || x - 1 == curr_drawn || x + 1 == curr_drawn {
            p2[row as usize][curr_drawn as usize] = '#';
        }
    }

    for inst in &input {
        let split_inst: Vec<&str> = inst.split(" ").collect();

        match split_inst[..] {
            ["noop"] => {
                draw(cycles2, x2, &mut p2);
                cycles2 += 1;
            }
            ["addx", val] => {
                for _i in 0..2 {
                    // p1 += calc_strength(cycles, x);
                    draw(cycles2, x2, &mut p2);
                    cycles2 += 1;
                }
                let val = val.parse::<i32>().unwrap();
                x2 += val;
            }
            _ => {}
        }
    }

    println!("P1 {}", p1);
    for row in p2 {
        for val in row {
            print!("{}", val);
        }
        println!("")
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let instructions: Vec<String> = fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect();
    return instructions;
}
