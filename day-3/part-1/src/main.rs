use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::io::Result as Result;
fn main() {

    let v = parse_input().unwrap();

    let mut answer: u32 = 0;
    let line_length = v[0].len();

    for i in 0..v.len() {
        let mut current_num = 0;
        let mut is_part = false;
        for j in 0..line_length {
            if v[i][j].is_numeric() {
                current_num = current_num * 10 + v[i][j].to_digit(10).unwrap();
                if ((i >= 1) && (v[i - 1][j] != '.')) {
                    is_part = true;
                } else if ((i + 1 < v.len()) && (v[i+1][j] != '.')) {
                    is_part = true;
                } 

                if j == line_length - 1 {
                    println!("hya!: {}", current_num);
                    if is_part {
                        answer += current_num;
                        current_num = 0;
                    } else {
                        current_num = 0;
                    }
                } 
            } else {
                if v[i][j] != '.' {
                    is_part = true;
                    if is_part {
                        answer += current_num;
                        current_num = 0;
                    }
                } else if (i >= 1) && (v[i - 1][j] != '.') {
                    is_part = true;
                    if is_part {
                        answer += current_num;
                        current_num = 0;
                    }
                } else if (i + 1 < v.len()) && (v[i + 1][j] != '.') {
                    is_part = true;
                    if is_part {
                        answer += current_num;
                        current_num = 0;
                    }
                } else {
                    if is_part {
                        answer += current_num;
                        current_num = 0;
                    }
                    current_num = 0;
                    is_part = false;
                }
            }
        }
    }
    
    println!("Answer: {}", answer);
}



fn parse_input() -> Result<Vec<Vec<char>>> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut v: Vec<String> = Vec::new();

    loop {
        line = String::new();
        match reader.read_line(&mut line) {
            // reader.read_line() outputs a result with the length of the extracted line in bytes.
            Ok(0) => break,
            Ok(_) => v.push(line.clone()),
            Err(_) => panic!("Error while reading file!"),
        }
    }

    // Maps each string to a vector of its characters.
    // .trim() to remove '\n'.
    let w: Vec<Vec<char>> = v.into_iter().map(|x| x.trim().chars().collect()).collect();

    Ok(w)
}
