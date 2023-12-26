use std::fs::File;
use std::io::*;
fn main() -> Result<()> {
    let mut f = File::open("part2.txt")?;
    let reader = BufReader::new(f);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    let mut buffer = [0; 10];

    // skip to the last 10 bytes of the file
    f.seek(SeekFrom::End(-10))?;

    // read up to 10 bytes
    let n = f.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..n]);

    let mut horiz = 0;
    let mut depth = 0;
    let mut x: i32;
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    if input_string == "forward {x}" {
        horiz += x;
    }
    Ok(())
}
