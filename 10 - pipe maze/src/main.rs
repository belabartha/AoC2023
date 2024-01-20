use std::{fs::File, io::{BufReader, BufRead}, vec};

type Position = (usize, usize);

struct PipeMap {
    map: Vec<Vec<char>>,
    num_columns: usize,
    num_rows: usize,
}

impl PipeMap {

    fn possible_moves(&self, from:&Position) -> (bool, bool, bool, bool) {
        let current: char = self.map[from.0][from.1];
        let may_move_north: bool = from.0 > 0 && (current == '|' || current == 'L' || current == 'J' || current == 'S');
        let may_move_south: bool = from.0 < self.num_rows - 1 && (current == '|' || current == '7' || current == 'F' || current == 'S');
        let may_move_east: bool = from.1 < self.num_columns - 1 && (current == 'F' || current == 'L' || current == '-' || current == 'S');
        let may_move_west: bool = from.1 > 0 && (current == 'J' || current == '7'  || current == '-' || current == 'S');

        let mut can_move_north = false;
        let mut can_move_south = false;
        let mut can_move_east = false;
        let mut can_move_west = false;

        if may_move_north {
            let north_char = self.map[from.0 - 1][from.1];
            if north_char == '|' || north_char == '7' || north_char == 'F' || north_char == 'S' {
                can_move_north = true;
            }
        }

        if may_move_south {
            let south_char = self.map[from.0 + 1][from.1];
            if south_char == '|' || south_char == 'L' || south_char == 'J' || south_char == 'S' {
                can_move_south = true;
            }
        }

        if may_move_east {
            let east_char = self.map[from.0][from.1 + 1];
            if east_char == '-' || east_char == '7' || east_char == 'J' || east_char == 'S' {
                can_move_east = true;
            }
        }

        if may_move_west {
            let west_char = self.map[from.0][from.1 - 1];
            if west_char == '-' || west_char == 'F' || west_char == 'L' || west_char == 'S' {
                can_move_west = true;
            }
        }

        (can_move_north, can_move_south, can_move_east, can_move_west)
    }

    // part 1
    fn move_one(&self, from: &Position, last: &Position) -> Position {
        let mut to: Position = from.clone();

        let (may_move_north, may_move_south, may_move_east, may_move_west) = self.possible_moves(from);
        
        let mut found: bool;

        // north
        if may_move_north {
            to = (from.0 - 1, from.1);
        }

        found = !(to.0 == last.0 && to.1 == last.1) && may_move_north;

        // south
        if may_move_south && !found {
            to = (from.0 + 1, from.1);
        }

        if !found {
            found = !(to.0 == last.0 && to.1 == last.1) && may_move_south;
        }

        // east
        if may_move_east && !found {
            to = (from.0, from.1 + 1);
        }

        if !found {
            found = !(to.0 == last.0 && to.1 == last.1) && may_move_east;
        }

        // west
        if may_move_west && !found {
            to = (from.0, from.1 - 1);
        }

        return to
    }
}


fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut trim_line: &str;

    let mut s_position: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];
    let mut current_line = 0;

    let mut linelen: usize = reader.read_line(&mut line)?;
    while linelen != 0 {
        trim_line = line.trim_end();
        let pos = trim_line.find('S');

        match pos {
            None => {} ,
            Some(val) => s_position = (current_line, val),
        }

        let row: Vec<char> = trim_line.chars().collect();
        map.push(row);

        current_line += 1;
        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    let num_rows = current_line;
    let num_cols = map[0].len();

    let mut clean_map: Vec<Vec<char>> = Vec::with_capacity(num_rows);
    clean_map.resize(num_rows, Vec::with_capacity(num_cols));
    for l in clean_map.as_mut_slice() {
        l.resize(num_cols, '.');
    }

    let pipe_map: PipeMap = PipeMap { map: map.clone(), num_columns: num_cols, num_rows: num_rows };

    let mut num_steps = 1;

    let mut prev_position = s_position.clone();
    let mut current_position = s_position.clone();
    
    // first step
    current_position = pipe_map.move_one(&current_position, &prev_position);
    
    let mut current_char = map[current_position.0][current_position.1];

    while current_char != 'S' {

        clean_map[current_position.0][current_position.1] = current_char;

        let next_position = pipe_map.move_one(&current_position, &prev_position);

        num_steps += 1;
        prev_position = current_position;
        current_position = next_position;
        current_char = map[current_position.0][current_position.1];
        //print!("{}", current_char);
    }

    println!("{}", (num_steps + 1) / 2);

    // part 2

    // replace S
    let (may_move_north, may_move_south, may_move_east, may_move_west) = pipe_map.possible_moves(&s_position);
    println!("n: {}, s: {}, e: {}, w: {}", may_move_north, may_move_south, may_move_east, may_move_west);
    let mut s_char = 'S';
    if may_move_north && may_move_east {
        s_char = 'L';
    } else if may_move_north && may_move_south {
        s_char = '|';
    } else if may_move_north && may_move_west {
        s_char = 'J';
    } else if may_move_east && may_move_south {
        s_char = 'F';
    } else if may_move_east && may_move_west {
        s_char = '-';
    } else if may_move_south && may_move_west {
        s_char = 'K';
    }

    clean_map[s_position.0][s_position.1] = s_char;

    let mut inside;
    let mut num_zones = 0;

    for l in clean_map.as_mut_slice() {
        inside = false;
        for c in l {
            if *c == '7' {
                *c = 'K';
            }
            if *c == '|' || *c == 'F' || *c == 'K' {
                inside = !inside;
            } else if *c == '.' && inside {
                *c = '*';
                num_zones += 1;
            }
        }
    }

    for l in clean_map.iter() {
        for c in l {
            print!("{}", c);
        }
        println!();
    }

    println!("num zones: {}", num_zones);

    Ok(())
}
