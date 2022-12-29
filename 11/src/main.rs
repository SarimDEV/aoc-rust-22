use std::fs;

fn main() {
    let (mut starting_items, operations, test) = read_input("input.txt");
    let mut starting_items_1 = starting_items.clone();
    let mut arr = vec![0; starting_items.len()];

    for _ in 0..20 {
        for i in 0..starting_items.len() {
            let (op, val) = &operations[i];
            let (test_val, truthy, falsey) = &test[i];
            arr[i] += starting_items[i].len();
            for j in 0..starting_items[i].len() {
                let item = starting_items[i][j];
                let mut new_worry_level = if *op == "+" {
                    item + *val
                } else if *op == "*" {
                    item * *val
                } else {
                    item * item
                };

                new_worry_level /= 3;

                let new_monkey;

                if new_worry_level % *test_val == 0 {
                    new_monkey = *truthy;
                } else {
                    new_monkey = *falsey;
                }

                starting_items[new_monkey as usize].push(new_worry_level);
            }

            starting_items[i].clear()
        }
    }

    let mut arr_1 = vec![0; starting_items_1.len()];
    let mut modulo: usize = 1;

    for (val, _, _) in &test {
        modulo *= *val;
    }

    for _ in 0..10000 {
        for i in 0..starting_items_1.len() {
            let (op, val) = &operations[i];
            let (test_val, truthy, falsey) = &test[i];
            arr_1[i] += starting_items_1[i].len();
            for j in 0..starting_items_1[i].len() {
                let item = starting_items_1[i][j];
                let mut new_worry_level = if *op == "+" {
                    item + *val
                } else if *op == "*" {
                    item * *val
                } else {
                    item * item
                };

                let new_monkey;

                if new_worry_level % *test_val == 0 {
                    new_monkey = *truthy;
                } else {
                    new_monkey = *falsey;
                }
                new_worry_level %= modulo;

                starting_items_1[new_monkey as usize].push(new_worry_level);
            }

            starting_items_1[i].clear()
        }
    }

    arr.sort();
    arr_1.sort();

    let p1 = arr.pop().unwrap() * arr.pop().unwrap();
    let p2 = arr_1.pop().unwrap() * arr_1.pop().unwrap();

    println!("P1 {}", p1);
    println!("P2 {}", p2);
}

fn read_input(filename: &str) -> (Vec<Vec<usize>>, Vec<(String, usize)>, Vec<(usize, usize, usize)>) {
    let input: Vec<Vec<String>> = fs::read_to_string(filename)
        .expect("failed to read file")
        .split("\n\n")
        .map(|l| l.lines().map(|q| q.to_string()).collect())
        .collect();

    let starting_items: Vec<Vec<usize>> = input
        .iter()
        .map(|i| {
            let (_, items) = i[1].split_once("Starting items: ").expect("mstestg");
            let items: Vec<usize> = items
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect();
            return items;
        })
        .collect();
    let operations: Vec<(String, usize)> = input
        .iter()
        .map(|i| {
            let (_, items) = i[2].split_once("Operation: new = old ").unwrap();
            let (op, val_str) = items.split_once(" ").unwrap();

            if val_str == "old" {
                return ("^2".to_string(), 1);
            }

            let val = val_str.parse::<usize>().unwrap_or(0);

            return (op.to_string(), val);
        })
        .collect();

    let test: Vec<(usize, usize, usize)> = input
        .iter()
        .map(|i| {
            let (_, val) = i[3].split_once("Test: divisible by ").unwrap();
            let (_, truthy) = i[4].split_once("If true: throw to monkey ").unwrap();
            let (_, falsey) = i[5].split_once("If false: throw to monkey ").unwrap();

            return (
                val.parse::<usize>().unwrap(),
                truthy.parse::<usize>().unwrap(),
                falsey.parse::<usize>().unwrap(),
            );
        })
        .collect();

    return (starting_items, operations, test);
}
