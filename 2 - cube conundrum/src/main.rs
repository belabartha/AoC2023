use std::{fs::File, io::{BufReader, BufRead}};

fn check_colour(count: u32, colour: &str) -> bool {
    match colour {
        "red" => count <= 12,
        "green" => count <= 13,
        "blue" => count <= 14,
        _ => false
    }
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;

    let mut reader = BufReader::new(file);

    let mut result = 0;
    let mut result2: u32 = 0;

    let mut line: String = String::new();
    let mut len: usize = reader.read_line(&mut line)?;
    while len != 0 {
        line = String::from(line.trim_end());
        let game_split: Vec<&str> = line.split(':').collect();
        let game_str = String::from(game_split[0]);
        let game: Vec<&str> = game_str.split(' ').collect();
        let game_number = game[1].to_string().parse::<u32>().unwrap();

        let cube_sets = String::from(game_split[1]);

        let mut possible: bool = true;
        let mut red_count: u32 = 0;
        let mut green_count: u32 = 0;
        let mut blue_count: u32 = 0;
        for cube_set in cube_sets.split(';') {
            let cube_set_str = String::from(cube_set); 
            let cubes: Vec<&str> = cube_set_str.split(',').collect();

            for cube in cubes {
                let cube_str = String::from(cube);
                let cube_details: Vec<&str> = cube_str.split(' ').collect();
                let count = String::from(cube_details[1]).parse::<u32>().unwrap();
                let colour = cube_details[2];
                let is_possible = check_colour(count, colour);
                if possible {
                    possible = is_possible;
                };

                match colour {
                    "red" => red_count = std::cmp::max(red_count, count),
                    "green" => green_count = std::cmp::max(green_count, count),
                    "blue" => blue_count = std::cmp::max(blue_count, count),
                    _ => (), 
                }
            }
        }
        if possible {
            result += game_number;
        }

        result2 += red_count * green_count * blue_count;

        line = String::from("");
        len = reader.read_line(&mut line)?;
    }

    println!("{}", result);
    println!("{}", result2);

    Ok(())
}
