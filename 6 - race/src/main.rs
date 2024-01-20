use std::{fs::File, io::{BufReader, BufRead}};

fn strvec_to_vec(numbers: &str, res: &mut Vec<u64>) {
    let str_numbers: Vec<&str> = numbers.split_whitespace().collect();

    for str_num in str_numbers {
        if !str_num.is_empty() {
            let num = str_num.parse::<u64>().unwrap();
            res.push(num);
        }
    }
}

fn str_to_num(numbers: &str) -> u64 {
    let mut ret: u64 = 0;

    let num: Vec<&str> = numbers.split_whitespace().collect();
    let full_str_num = num.join("");

    for c in full_str_num.chars() {
        ret = ret * 10 + c.to_digit(10).unwrap() as u64;
    }

    ret
}

fn get_num_win_races(time: u64, dist: u64) -> u64 {
    let result: u64;

    let mut start: u64 = f32::round(((time + dist) as f32) / time as f32) as u64;

    while (time - start) * start <= dist {
        start += 1;
    }

    let end:u64 = time - start;

    result = end - start + 1;

    result
}

fn main()-> std::io::Result<()> {
    let file: File = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut linelen: usize = reader.read_line(&mut line)?;

    let mut race_time: Vec<u64> = vec![];
    let mut race_dist: Vec<u64> = vec![];

    // part 2
    let mut p2_race_time: u64 = 0;
    let mut p2_race_dist: u64 = 0;

    while linelen != 0 {
        let trim_line: &str = line.trim_end();
        let numbers = trim_line.split_at(9).1;
        if trim_line.starts_with("Time") {
            strvec_to_vec(numbers, &mut race_time);
            p2_race_time = str_to_num(numbers);
        } else if trim_line.starts_with("Dist") {
            strvec_to_vec(numbers, &mut race_dist);
            p2_race_dist = str_to_num(numbers);
        }

        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    let mut result: u64 = 1;
    
    for race in 0..race_time.len() {
        let race_result = get_num_win_races(race_time[race], race_dist[race]);
        result *= race_result;
    }

    println!("{}", result);

    // part 2
    let result2: u64;
    result2 = get_num_win_races(p2_race_time, p2_race_dist);

    println!("{}", result2);

    Ok(())
}
