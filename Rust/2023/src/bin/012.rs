use std::collections::HashMap;
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

    // Hack to read overlapping numbers in string format.
    let mut numbers_map: HashMap<&str, &str> = HashMap::new();
    numbers_map.insert("one", "o1ne");
    numbers_map.insert("two", "tw2o");
    numbers_map.insert("three", "th3ree");
    numbers_map.insert("four", "fo4ur");
    numbers_map.insert("five", "fiv5e");
    numbers_map.insert("six", "si6x");
    numbers_map.insert("seven", "sev7en");
    numbers_map.insert("eight", "ei8ght");
    numbers_map.insert("nine", "nin9e");

    let mut numbers: Vec<String> = Vec::new();
    let mut sum = 0;
    for text in lines {
        let mut new_line = text;
        for (key, value) in numbers_map.iter() {
            if new_line.contains(key) {
                new_line = new_line.replace(key, value)
            }
        }
        numbers.push(new_line.replace(
            [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            "",
        ))
    }
    for s in numbers {
        let mut to_be_summed = Vec::new();
        for c in s.chars() {
            if c.is_numeric() {
                to_be_summed.push(c);
            }
        }
        let first = to_be_summed.first().unwrap().to_string();
        let last = to_be_summed.last().unwrap().to_string();
        let together = first.clone() + &last;
        sum += together.parse::<i32>().unwrap();
    }
    println!("{:?}", sum);

    Ok(())
}
