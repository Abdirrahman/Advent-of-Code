use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("./part1.txt")?;
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    let mut _counter = 1;
    let mut sum: Vec<i32> = Vec::new();
    let mut counter = 1;
    for i in lines {
        let mut score = 0;
        let red: Vec<_> = i.match_indices("red").collect();
        for item in red {
            let chars: Vec<_> = i.chars().collect();
            let prefix_chars: &[char] = &chars[item.0 - 3..item.0];
            let mut numbers: String = String::new();
            for i in prefix_chars {
                if i.is_digit(10) {
                    numbers.push(*i);
                    if numbers.parse::<i32>().unwrap() > 12 {
                        score += 1
                    }
                }
            }
        }
        let green: Vec<_> = i.match_indices("green").collect();
        for item in green {
            let chars: Vec<_> = i.chars().collect();
            let prefix_chars: &[char] = &chars[item.0 - 3..item.0];
            let mut numbers: String = String::new();
            for i in prefix_chars {
                if i.is_digit(10) {
                    numbers.push(*i);
                    if numbers.parse::<i32>().unwrap() > 13 {
                        score += 1
                    }
                }
            }
        }
        let blue: Vec<_> = i.match_indices("blue").collect();
        for item in blue {
            let chars: Vec<_> = i.chars().collect();
            let prefix_chars: &[char] = &chars[item.0 - 3..item.0];
            let mut numbers: String = String::new();
            for i in prefix_chars {
                if i.is_digit(10) {
                    numbers.push(*i);
                    if numbers.parse::<i32>().unwrap() > 14 {
                        score += 1
                    }
                }
            }
        }

        if score == 0 {
            sum.push(counter)
        }
        println!("{:?}", sum.iter().sum::<i32>());
        counter += 1;
    }

    Ok(())
}
