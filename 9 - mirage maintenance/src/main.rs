use std::{fs::File, io::{BufReader, BufRead}};
fn predict(line: &Vec<i64>) -> i64 {
    let mut ret: i64 = 0;
    let mut predictor: Vec<i64> = vec![];

    let mut level = 1;
    let mut current_line: Vec<i64> = line.clone();

    while current_line[0] != 0 || current_line[1] != 0 {
        predictor.push(current_line[0]);
        for pos in 0..level {
            current_line[level - pos - 1] = current_line[level - pos - 1] - current_line[level - pos];
        }
        // println!("{:?}", current_line);
        level += 1;
    }

    // println!("predictor: {:?}", predictor);
    for num in predictor {
        ret += num;
    }

    //print!("{:?} - {}", line, ret);

    ret
}

fn predict2(line: &Vec<i64>) -> i64 {
    let mut ret: i64;
    let mut predictor: Vec<i64> = vec![];

    let mut level = 1;
    let mut current_line: Vec<i64> = line.clone();
    current_line.reverse();

    while current_line[0] != 0 || current_line[1] != 0 {
        predictor.push(current_line[0]);
        for pos in 0..level {
            current_line[level - pos - 1] = current_line[level - pos] - current_line[level - pos - 1];
        }
        // println!("{:?}", current_line);
        level += 1;
    }

    predictor.pop();
    let mut ret: i64 = 0;
    for num in &predictor {
        ret = num - ret;
    }

    println!("predictor: {:?} - {}", predictor, ret);

    ret
}

fn safety2(line: &Vec<i64>) -> i64 {
    let mut prev_line: Vec<i64> = line.clone();
    prev_line.reverse();
    let mut curr_line: Vec<i64> = vec![];
    let mut predictor: Vec<i64> = vec![];

    let mut done: bool = false;
    while !done {
        done = true;
        predictor.push(prev_line[0]);
        for pos in 0..prev_line.len() - 1 {
            curr_line.push(prev_line[pos + 1] - prev_line[pos])
        }

        for num in &curr_line {
            if *num != 0 {
                done = false;
                break;
            }
        }
        println!("{:?}", curr_line);
        prev_line = curr_line.clone();
        curr_line.clear();
    }

    // println!("predictor: {:?}", predictor);
    predictor.reverse();

    let mut ret = 0;
    for num in &predictor {
        ret = num - ret;
    }
    println!("predictos: {:?} - {}", predictor, ret);
    //println!(" - {}", ret);

    ret

}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut trim_line: &str;

    let mut line_vec: Vec<i64>;
    let mut data: Vec<Vec<i64>> = vec![];

    let mut linelen: usize = reader.read_line(&mut line)?;
    while linelen != 0 {
        trim_line = line.trim_end();
        let str_nums: Vec<&str> = trim_line.split_whitespace().collect();
        line_vec = str_nums.iter().map(|str_num| str_num.parse::<i64>().unwrap()).rev().collect();
        data.push(line_vec.clone());

        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    let mut ret = 0;
    let mut ret2: i64 = 0;

    for l in &data {
        ret += predict(l);
    }

    let mut sfret: i64 = 0;

    for l in &data {
        //ret2 += predict2(l);
        sfret += safety2(l);
        // if ret2 != sfret {
        //     println!();
        // }
    }

    println!("{} - {}", ret, sfret);

    Ok(())
}
