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
    let mut vec = Vec::new();
    for text in lines {
        vec.push(text.replace(
            [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            "",
        ));
    }
    let mut numbers = Vec::new();
    for x in &vec {
        let first = x.chars().nth(0).unwrap().to_string().to_owned();
        let last = &x.chars().nth(x.len() - 1).unwrap().to_string();

        let together = first.clone() + last;
        numbers.push(together);
    }
    let mut to_be_summed: Vec<Result<i64, std::num::ParseIntError>> = numbers
        .into_iter()
        .map(|x| x.parse::<i64>())
        .collect::<Vec<_>>();

    let sum: i64 = to_be_summed.iter().flatten().sum::<i64>();

    println!("{:?}", sum);

    Ok(())
}
