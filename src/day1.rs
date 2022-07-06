use std::fs::File;
use std::io::*;

fn main() -> Result<()> {
    let f = File::open("foo.txt")?;
    let reader = BufReader::new(f);
    let nums: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    println!("{:?}", &nums);
    let mut counter = 0;
    for (index, n) in nums.iter().enumerate() {
        if nums[index] + nums[index + 1] + nums[index + 2]
            < nums[index + 1] + nums[index + 2] + nums[index + 3]
        {
            println!("comparing and {} and {}", nums[index], nums[index + 1]);
            counter += 1;
        }
        println!("Counter Value is {}", counter);
        println!("The next number was {}", nums[index + 1]);
    }
    Ok(())
}
