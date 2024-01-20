use std::{fs::File, io::{BufReader, BufRead}};

// fn is_symmbol_close(first: Option<&char>, second: Option<&char>, third: Option<&char>) -> bool { 
//     first.is_some() && !first.unwrap().is_ascii_digit() && !first.unwrap().eq(&'.') && !first.unwrap().eq(&'\n') && !first.unwrap().eq(&'\r') ||
//         second.is_some() && !second.unwrap().is_ascii_digit() && !second.unwrap().eq(&'.') && !second.unwrap().eq(&'\n') && !second.unwrap().eq(&'\r') ||
//         third.is_some() && !third.unwrap().is_ascii_digit() && !third.unwrap().eq(&'.') && !third.unwrap().eq(&'\n') && !third.unwrap().eq(&'\r')
// }

// fn check_line(prev_line_option: Option<&String>, line: &String, next_line_option: Option<&String>) -> u32 {
//     let mut ret = 0;

//     let mut number: u32 = 0;

//     let is_prev_line_available: bool = prev_line_option.is_some();
//     let is_next_line_available = next_line_option.is_some();

//     let prev_characters: Vec<char> = prev_line_option.unwrap_or(&String::new()).chars().collect();
//     let characters:Vec<char> = line.chars().collect();
//     let next_characters: Vec<char> = next_line_option.unwrap_or(&String::new()).chars().collect();
//     let mut is_adjancent = false;

//     for (pos, character) in characters.iter().enumerate() {
//         if character.is_ascii_digit() {
//             number = number * 10 + character.to_digit(10).unwrap();
//             if !is_adjancent {
//                 let is_next_adjancent = if is_next_line_available {
//                     let next_first: Option<&char> = if pos < 2 { None } else { Some(&next_characters[pos - 1]) };
//                     let next_third: Option<&char> = if pos > characters.len() - 2 { None } else { Some(&next_characters[pos + 1]) };
//                     is_symmbol_close(next_first, Some(&next_characters[pos]), next_third)
//                 } else {
//                     false
//                 };

//                 let is_prev_adjancent = if is_prev_line_available {
//                     let prev_first: Option<&char> = if pos < 2 { None } else { Some(&prev_characters[pos - 1]) };
//                     let prev_third: Option<&char> = if pos > characters.len() - 2 { None } else { Some(&prev_characters[pos + 1])};
//                     is_symmbol_close(prev_first, Some(&prev_characters[pos]), prev_third)
//                 } else {
//                     false
//                 };

//                 let is_line_adjancent: bool = {
//                     let prev: Option<&char> = if pos > 1 { Some(&characters[pos - 1]) } else { None };
//                     let next: Option<&char> = if pos < characters.len() - 2 { Some(&characters[pos + 1]) } else { None };
//                     is_symmbol_close(prev, None, next)
//                 };

//                 is_adjancent = is_next_adjancent || is_prev_adjancent || is_line_adjancent;
//             }
//         } else {
//              if !character.is_ascii_digit() {
//                 if number != 0 && is_adjancent {
//                     ret += number;
//                 }
//             }
//             is_adjancent = false;
//             number = 0;
//         }
//     }

//     ret
// }

#[derive(Debug)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
}

fn line_to_numbers(line: &Vec<char>) -> Vec<Number> {
    let mut ret: Vec<Number> = vec![];

    let mut current = Number { value: 0, start: usize::MAX, end: usize::MAX };

    for (pos, character) in line.iter().enumerate() {
        if character.is_ascii_digit() {
            current.value = current.value * 10 + character.to_digit(10).unwrap();
            if current.start == usize::MAX {
                current.start = pos;
            }
            current.end = pos;
        } else {
            if current.value != 0 {
                ret.push(current);
                current = Number { value: 0, start: usize::MAX, end: usize::MAX };
            }
        }
    }

    if current.value != 0 {
        ret.push(current);
    }
    // println!("{:?}", ret);

    ret
}

fn check_line3(prev_line_option: Option<&String>, line: &String, next_line_option: Option<&String>) -> u32 {
    let mut ret = 0;

    let prev_characters: Vec<char> = prev_line_option.unwrap_or(&String::new()).chars().collect();
    let characters:Vec<char> = line.chars().collect();
    let next_characters: Vec<char> = next_line_option.unwrap_or(&String::new()).chars().collect();

    let prev_numbers = line_to_numbers(&prev_characters);
    let numbers = line_to_numbers(&characters);
    let next_numbers = line_to_numbers(&next_characters);

    let mut adjancent_numbers: Vec<&Number> = vec![];

    for (pos, character) in characters.iter().enumerate() {
        if character.eq(&'*') {
            // print!("{} - ", pos);
            for (_, num) in prev_numbers.iter().enumerate() {
                if num.end == pos - 1 ||
                    pos >= num.start && pos <= num.end ||
                    num.start == pos + 1 {
                    adjancent_numbers.push(num);
                    // print!(" {:?}", num);
                }
            }
            for (_, num) in next_numbers.iter().enumerate() {
                if num.end == pos - 1 ||
                    pos >= num.start && pos <= num.end ||
                    num.start == pos + 1 {
                    adjancent_numbers.push(num);
                    // print!(" {:?}", num);
                }
            }
            for (_, num) in numbers.iter().enumerate() {
                if num.start == pos + 1 || num.end == pos - 1 {
                    adjancent_numbers.push(num);
                    // print!(" {:?}", num);
                }
            }

            if adjancent_numbers.len() == 2 {
                ret += adjancent_numbers[0].value * adjancent_numbers[1].value;
            }
            adjancent_numbers = vec![];
            // println!();
        }
    }

    // println!();

    ret
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;
    let mut reader = BufReader::new(file);
    
    //let mut result = 0;
    let mut result2: u32 = 0;
    let mut prev_line: String = String::new();
    let mut line: String = String::new();
    let mut next_line: String = String::new();
    reader.read_line(&mut prev_line)?;
    reader.read_line(&mut line)?;
    reader.read_line(&mut next_line)?;

    prev_line = String::from(prev_line.trim());
    line = String::from(line.trim());
    next_line = String::from(next_line.trim());

    //result += check_line(None, &prev_line, Some(&line));
    result2 += check_line3(None, &prev_line, Some(&line));

    while !next_line.is_empty() {
        // next_line = String::from(next_line.trim_end());

        //result += check_line(Some(&prev_line), &line, Some(&next_line));
        result2 += check_line3(Some(&prev_line), &line, Some(&next_line));

        prev_line = line.clone();
        line = next_line.clone();
        next_line = String::new();
        reader.read_line(&mut next_line)?;
        next_line = String::from(next_line.trim());
    }

    //result += check_line(Some(&prev_line), &line, None);
    result2 += check_line3(Some(&prev_line), &line, None);

    //println!("{}", result);
    println!("{}", result2);

    Ok(())
}
