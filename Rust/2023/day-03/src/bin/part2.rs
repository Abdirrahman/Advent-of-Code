use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("./part1.txt")?;
    let coords: Vec<((usize, usize), char)> = array_2d(f);
    let _numbers: HashMap<(usize, usize), char> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut gears: HashMap<(usize, usize), char> = HashMap::new();
    for line in &coords {
        if !line.1.is_digit(10) && line.1 != '.' {
            symbols.insert((line.0 .0, line.0 .1), line.1);
        }
    }

    for line in &coords {
        if line.1 == '*' {
            gears.insert((line.0 .0, line.0 .1), line.1);
        }
    }

    let mut tbs: Vec<i32> = Vec::new();
    let mut store_coords: Vec<(Vec<(usize, usize)>, i32)> = Vec::new();
    let mut store: Vec<(usize, usize)> = Vec::new();
    let mut store_digits = Vec::new();
    for line in coords {
        for items in store.clone() {
            if items.0 == 139 {
                if check_symbol(store.clone(), gears.clone()) {
                    store_coords.push((
                        store.clone(),
                        store_digits
                            .clone()
                            .into_iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap(),
                    ));
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
                store_coords.push((
                    store.clone(),
                    store_digits
                        .clone()
                        .into_iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                ));
                store.clear();
                store_digits.clear();
            }
            store.clear();
            store_digits.clear();
        }
    }
    count_gears(store_coords, gears);
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

fn count_gears(
    digits_coords: Vec<(Vec<(usize, usize)>, i32)>,
    gears: HashMap<(usize, usize), char>,
) {
    let mut gear_pair: Vec<i32> = Vec::new();
    let mut sum_pairs: Vec<i32> = Vec::new();
    for (key, _value) in gears.into_iter() {
        for i in digits_coords.clone().into_iter() {
            if check_gear(i.0, key) {
                gear_pair.push(i.1);
            }
        }

        if gear_pair.len() == 2 {
            sum_pairs.push(gear_pair.iter().product());
            gear_pair.clear()
        }
        gear_pair.clear()
    }
    println!("Sum of gear pairs: {:?}", sum_pairs.iter().sum::<i32>())
}

fn check_gear(vec: Vec<(usize, usize)>, key: (usize, usize)) -> bool {
    for line in vec {
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
            if co == key {
                return true;
            }
        }
        for co in up_yaxis {
            if co == key {
                return true;
            }
        }
        for co in down_yaxis {
            if co == key {
                return true;
            }
        }
    }
    return false;
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
