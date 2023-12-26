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

    let mut sum: Vec<i32> = Vec::new();

    for line in lines {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        let red: Vec<_> = line.match_indices("red").collect();
        let mut num = 0;
        for item in red {
            let chars: Vec<_> = line.chars().collect();
            let prefix_chars: &[char] = &chars[item.0 - 3..item.0];
            let mut numbers: String = String::new();
            for i in prefix_chars {
                if i.is_digit(10) {
                    numbers.push(*i);

                    if num < numbers.parse::<i32>().unwrap() {
                        num = numbers.parse::<i32>().unwrap();
                    }
                }
            }
            red_count = num
        }
        let green: Vec<_> = line.match_indices("green").collect();
        let mut num = 0;
        for item in green {
            let chars: Vec<_> = line.chars().collect();
            let prefix_chars: &[char] = &chars[item.0 - 3..item.0];
            let mut numbers: String = String::new();
            for i in prefix_chars {
                if i.is_digit(10) {
                    numbers.push(*i);

                    if num < numbers.parse::<i32>().unwrap() {
                        num = numbers.parse::<i32>().unwrap();
                    }
                }
            }
            green_count = num
        }

        let blue: Vec<_> = line.match_indices("blue").collect();
        let mut num = 0;
        for item in blue {
            let chars: Vec<_> = line.chars().collect();
            let prefix_chars: &[char] = &chars[item.0 - 3..item.0];

            let mut numbers: String = String::new();
            for i in prefix_chars {
                if i.is_digit(10) {
                    numbers.push(*i);
                    if num < numbers.parse::<i32>().unwrap() {
                        num = numbers.parse::<i32>().unwrap();
                    }
                }
            }
            blue_count = num
        }
        sum.push(red_count * green_count * blue_count)
    }
    println!("{:?} ", sum.iter().sum::<i32>());
    Ok(())
}
