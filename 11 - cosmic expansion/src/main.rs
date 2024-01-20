use std::{fs::File, io::{BufReader, BufRead }};

struct Universe {
    map: Vec<Vec<char>>,
}

impl Universe {
    fn add_row(&mut self, row: &String) {
        self.map.push(row.chars().collect());
    }

    fn get_galaxy_distances(&self, multiplier: usize) -> usize {
        let mut galaxies: Vec<(usize, usize)> = vec![];

        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == '#' {
                    galaxies.push((row, col));
                }
            }
        }

        let mut empty_rows: Vec<usize> = vec![];
        let mut has_galaxy: bool;
        for row in 0..self.map.len() {
            has_galaxy = true;
            for col in 0..self.map[0].len() {
                if self.map[row][col] == '#' {
                    has_galaxy = false;
                    break;
                }
            }
            if has_galaxy {
                empty_rows.push(row);
            }
        }

        let mut empty_cols: Vec<usize> = vec![];
        
        for col in 0..self.map[0].len() {
            has_galaxy = true;
            for row in 0..self.map.len() {
                if self.map[row][col] == '#' {
                    has_galaxy = false;
                    break;
                }
            }
            if has_galaxy {
                empty_cols.push(col);
            }
        }

        let mut dist:usize = 0;

        for g1_pos in 0..galaxies.len() {
            for g2_pos in g1_pos + 1..galaxies.len() {

                let g1 = galaxies[g1_pos];
                let g2 = galaxies[g2_pos];

                let before_g1_rows = num_before(&empty_rows, g1.0, multiplier);
                let before_g2_rows = num_before(&empty_rows, g2.0, multiplier);
                let before_g1_cols = num_before(&empty_cols, g1.1, multiplier);
                let before_g2_cols = num_before(&empty_cols, g2.1, multiplier);

                //print!("{} {} {} {}", before_g1_rows, before_g1_cols, before_g2_rows, before_g2_cols);
                //print!(" --- g1 {:?}, g2 {:?}", g1, g2);

                let g_dist = (galaxies[g2_pos].0 + before_g2_rows).abs_diff(galaxies[g1_pos].0 + before_g1_rows) +
                    (galaxies[g2_pos].1 + before_g2_cols).abs_diff(galaxies[g1_pos].1 + before_g1_cols);

                dist += g_dist;
                // println!(" --- dist: {}", g_dist);
            }
        }

        return dist;
    }

}

fn num_before(vec: &Vec<usize>, value: usize, multiplier: usize) -> usize {
    let mut ret = bin_search(vec, 0, vec.len(), value);

    if ret > 0 {
        ret = ret * (multiplier - 1);
    }

    ret
}

fn bin_search(vec: &Vec<usize>, start: usize, end: usize, value: usize) -> usize {
    let mid = (end + start) / 2;

    if start >= end {
        return start;
    }

    if value < vec[mid] {
        return bin_search(vec, start, mid, value);
    } else if value > vec[mid] {
        return bin_search(vec, mid + 1, end, value)
    } else {
        return mid - 1;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut trim_line: &str;

    let mut line_len = reader.read_line(&mut line)?;

    let mut universe: Universe = Universe { map: vec![] };

    while line_len != 0 {
        trim_line = line.trim_end();

        universe.add_row(&String::from(trim_line));

        line.clear();
        line_len = reader.read_line(&mut line)?;
    }

    for row in 0..universe.map.len() {
        for col in 0..universe.map[0].len() {
            print!("{}", universe.map[row][col]);
        }
        println!();
    }

    let res1 = universe.get_galaxy_distances(2);

    println!("dist1: {}", res1);

    let res2 = universe.get_galaxy_distances(1000000);

    println!("dist2: {}", res2);

    Ok(())

}
