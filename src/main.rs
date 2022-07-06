use std::io::*;

fn main() {
    let mut horiz = 0;
    let mut depth = 0;
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    if input_string == "forward {x}" {
        horiz += x;
    }
}
