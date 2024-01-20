#![allow(unused)]
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader};

fn string_to_digit(str: &String) -> u32 {
    if str.contains("zero") {
        return 0;
    } else if str.contains("one") {
        return 1;
    } else if str.contains("two") {
        return 2;
    } else if str.contains("three") {
        return 3;
    } else if str.contains("four") {
        return 4;
    } else if str.contains("five") {
        return 5;
    } else if str.contains("six") {
        return 6;
    } else if str.contains("seven") {
        return 7;
    } else if str.contains("eight") {
        return 8;
    } else if str.contains("nine") {
        return 9;
    }

    return 10;
}

fn reverse_string_to_digit(str: &String) -> u32 {
    if str.contains("zero") {
        return 0;
    } else if str.contains("eno") {
        return 1;
    } else if str.contains("owt") {
        return 2;
    } else if str.contains("eerht") {
        return 3;
    } else if str.contains("ruof") {
        return 4;
    } else if str.contains("evif") {
        return 5;
    } else if str.contains("xis") {
        return 6;
    } else if str.contains("neves") {
        return 7;
    } else if str.contains("thgie") {
        return 8;
    } else if str.contains("enin") {
        return 9;
    }

    return 10;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut result: u32 = 0;
    let mut line_result: u32 = 0;

    let mut len: usize = reader.read_line(&mut line)?;
    while len != 0 {
        line_result = 0;
        let mut word = String::new();
        for character in line.chars() {
            if character.is_ascii_digit() {
                let digit = character.to_digit(10).unwrap();
                line_result = digit;
                break;
            } else {
                word.push(character);
                let value = string_to_digit(&word);
                if (value < 10) {
                    line_result = value;
                    break;
                }
            }
        }
        word = String::from("");
        for character in line.chars().rev() {
            if character.is_ascii_digit() {
                let digit = character.to_digit(10).unwrap();
                line_result = line_result * 10 + digit;
                break;
            } else if (character.is_alphanumeric()) {
                word.push(character);
                let value = reverse_string_to_digit(&word);
                if (value < 10) {
                    line_result = line_result * 10 + value;
                    break;
                }
            }
        }
        // println!("{}", line_result);
        result += line_result;
        line = String::from("");
        len = reader.read_line(&mut line)?;
    }
    println!("result: {:?}", result);
    Ok(())
}
