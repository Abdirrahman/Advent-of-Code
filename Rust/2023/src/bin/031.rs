use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("./part1.txt")?;
    let coords = array_2d(f);
    let _numbers: HashMap<(usize, usize), char> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    for line in &coords {
        if !line.1.is_digit(10) && line.1 != '.' {
            symbols.insert((line.0 .0, line.0 .1), line.1);
        }
    }

    let mut tbs: Vec<i32> = Vec::new();
    let mut store: Vec<(usize, usize)> = Vec::new();
    let mut store_digits = Vec::new();
    for line in coords {
        for items in store.clone() {
            if items.0 == 139 {
                println!(" 139 {:?}", store);
                if check_symbol(store.clone(), symbols.clone()) {
                    tbs.push(
                        store_digits
                            .clone()
                            .into_iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap(),
                    );
                    store.clear();
                    store_digits.clear();
                }
                store.clear();
                store_digits.clear();
            }
        }

        if line.1.is_digit(10) {
            store.push(line.0);
            store_digits.push(line.1)
        } else if !store.is_empty() {
            if check_symbol(store.clone(), symbols.clone()) {
                tbs.push(
                    store_digits
                        .clone()
                        .into_iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                );
                store.clear();
                store_digits.clear();
            }
            store.clear();
            store_digits.clear();
        }
    }

    println!("{:?}", tbs.iter().sum::<i32>());

    Ok(())
}

fn array_2d(input: File) -> Vec<((usize, usize), char)> {
    let reader = BufReader::new(input);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    let mut grid = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for x in line.char_indices() {
            grid.push(((x.0, y), x.1));
        }
    }
    return grid;
}

fn check_symbol(store: Vec<(usize, usize)>, symbol: HashMap<(usize, usize), char>) -> bool {
    for line in store.iter() {
        let x_axis: [(usize, usize); 2] = [
            (line.0.checked_sub(1).unwrap_or(10000), line.1),
            (line.0 + 1, line.1),
        ];
        let up_yaxis: [(usize, usize); 3] = [
            (line.0, line.1.checked_sub(1).unwrap_or(10000)),
            (
                line.0.checked_sub(1).unwrap_or(10000),
                line.1.checked_sub(1).unwrap_or(10000),
            ),
            (line.0 + 1, line.1.checked_sub(1).unwrap_or(10000)),
        ];
        let down_yaxis: [(usize, usize); 3] = [
            (line.0, line.1 + 1),
            (line.0.checked_sub(1).unwrap_or(10000), line.1 + 1),
            (line.0 + 1, line.1 + 1),
        ];
        for co in x_axis {
            if symbol.keys().any(|&x| x == co) {
                return true;
            }
        }
        for co in up_yaxis {
            if symbol.keys().any(|&x| x == co) {
                return true;
            }
        }
        for co in down_yaxis {
            if symbol.keys().any(|&x| x == co) {
                return true;
            }
        }
    }
    return false;
}
