use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("./part1.txt")?;
    let coords = array_2d(f);

    // let reader = BufReader::new(f);
    // let lines: Vec<String> = reader
    //     .lines()
    //     .map(|line| line.unwrap().parse::<String>().unwrap())
    //     .collect();
    let _numbers: HashMap<(usize, usize), char> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();

    // println!("{:?}", coords);
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
        // println!("{:?}", store);
        if line.1.is_digit(10) {
            store.push(line.0);
            store_digits.push(line.1)
        } else if !store.is_empty() {
            // println!("{:?}", store);
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
            // println!("{:?}", store_digits);
            store.clear();
            store_digits.clear();
            // println!("cleared");
        }
    }

    println!("{:?}", tbs.iter().sum::<i32>());
    // println!("{:?}", tbs);

    // for line in coords {}
    // for (i, line) in lines.iter().enumerate() {}
    // println!("{:?}", coords);
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
// 521515
// 1634187
// Solved?: 1634187
// Check is digit then push to vec, pass the vec to symbol check
// combine vec chars and push to sum vec

// TODO:
// check all axis and put if statements
// (x, y)

// fn check_adjacent(line: (usize, usize)) {
//     let x_axis = [
//         (line.0.checked_sub(1).unwrap_or(0), line.1),
//         (line.0 + 1, line.1),
//     ];
// }
// <((usize, usize), char) as IntoIterator>::Item
fn check_symbol(store: Vec<(usize, usize)>, symbol: HashMap<(usize, usize), char>) -> bool {
    for line in store.iter() {
        // println!("{:?}", line);
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
        // println!("{:?}", x_axis);
        // println!("{:?}", up_yaxis);
        // println!("{:?}", down_yaxis);
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
        // return false;
    }
    return false;
}

// fn check_adjacent(line: ((usize, usize), char), symbol: HashMap<(usize, usize), char>) -> bool {
//     // for x axis move right by 1 if not 0 also move left by one.

//     // let xaxis = [(line.0 .0 + 1, line.0 .1), (line.0 .0.checked_sub(1).unwrap(), line.0 .1)];
//     let xaxis = [
//         (line.0 .0.checked_add(1).unwrap(), line.0 .1),
//         (line.0 .0.checked_sub(1).unwrap(), line.0 .1),
//     ];

//     let down_yaxis = [
//         (line.0 .0, line.0 .1.checked_add(1).unwrap()),
//         (
//             line.0 .0.checked_add(1).unwrap(),
//             line.0 .1.checked_add(1).unwrap(),
//         ),
//         (
//             line.0 .0.checked_sub(1).unwrap(),
//             line.0 .1.checked_add(1).unwrap(),
//         ),
//         (
//             line.0 .0.checked_add(2).unwrap(),
//             line.0 .1.checked_add(1).unwrap(),
//         ),
//         (
//             line.0 .0.checked_sub(2).unwrap(),
//             line.0 .1.checked_add(1).unwrap(),
//         ),
//     ];

//     let up_yaxis = [
//         (line.0 .0, line.0 .1.checked_sub(1).unwrap()),
//         (
//             line.0 .0.checked_add(1).unwrap(),
//             line.0 .1.checked_sub(1).unwrap(),
//         ),
//         (
//             line.0 .0.checked_sub(1).unwrap(),
//             line.0 .1.checked_sub(1).unwrap(),
//         ),
//         (
//             line.0 .0.checked_add(2).unwrap(),
//             line.0 .1.checked_sub(1).unwrap(),
//         ),
//         (
//             line.0 .0.checked_sub(2).unwrap(),
//             line.0 .1.checked_sub(1).unwrap(),
//         ),
//     ];

//     if line.0 .0 == 0 {
//         if symbol
//             .keys()
//             .any(|&x| x == (line.0 .0.checked_add(1).unwrap(), line.0 .1))
//         {
//             return true;
//         }
//         for co in down_yaxis {
//             if symbol.keys().any(|&x| x == co) {
//                 return true;
//             }
//             for co in up_yaxis {
//                 if symbol.keys().any(|&x| x == co) {
//                     return true;
//                 }
//             }
//         }
//     }

//     if line.0 .1 == 0 {
//         for co in xaxis {
//             if symbol.keys().any(|&x| x == co) {
//                 return true;
//             }
//         }

//         for co in down_yaxis {
//             if symbol.keys().any(|&x| x == co) {
//                 return true;
//             }
//         }
//     }

//     for co in xaxis {
//         if symbol.keys().any(|&x| x == co) {
//             return true;
//         }
//     }

//     for co in down_yaxis {
//         if symbol.keys().any(|&x| x == co) {
//             return true;
//         }
//     }
//     for co in up_yaxis {
//         if symbol.keys().any(|&x| x == co) {
//             return true;
//         }
//     }

//     return false;

//     // for y axis move down 1 and if not 0 move up one.
//     // also here move x axis by 1L, 0, 1R.
// }

// fn check_for_symbol(
//     grid: Vec<((usize, usize), char)>,
//     symbols: HashMap<(usize, usize), char>,
// ) -> bool {
//     if let Some(line) = grid.into_iter().next() {
//         for line in grid {
//             if symbols.contains_key(&line.0) {
//                 println!("{:?}", line);
//             }
//         }
//         return true;
//     } else {
//         return false;
//     }
// for line in grid {
//     println!("{:?}", line);
//     return true;
// }
// return false;
// }

// X,Y coords by iter
// hashmap/btree for coords of symbols?
// check num adjacent to symbol
// get nums adjacent to that num and store
