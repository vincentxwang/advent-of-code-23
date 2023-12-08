use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let string = line.unwrap();
        let chars = string.chars();
        for chr in chars.clone() {
            if chr.is_numeric() {
                count += 10 * chr.to_digit(10).unwrap();
                break;
            }
        }
        for chr in chars.rev() {
            if chr.is_numeric() {
                count += chr.to_digit(10).unwrap();
                break;
            }
        }
    }

    println!("{count}");

    Ok(())
}
