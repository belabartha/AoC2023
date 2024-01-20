use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, usize::MAX, time::Instant };
use regex::Regex;

#[derive(Debug)]
struct Node {
    name: String,
    left: usize,
    right: usize,
}

fn main() -> std::io::Result<()> {

    let now = Instant::now();

    let file: File = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut trim_line: &str;
    
    // read line 0 - turns
    let mut instructions_string: String = String::new();
    reader.read_line(&mut instructions_string)?;

    let instructions: Vec<char> = instructions_string.trim_end().chars().collect();

    // skip line
    reader.read_line(&mut line)?;
    line.clear();

    let mut path: HashMap<String, (String, String, usize)> = HashMap::new();
    let splitter: Regex = Regex::new(r"= \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();

    let mut graph: Vec<Node> = vec![];

    // read path
    let mut linelen: usize = reader.read_line(&mut line)?;
    while linelen != 0 {
        trim_line = line.trim_end();

        let (node_name, remains) = trim_line.split_at(3);
        let lr: regex::Captures<'_> = splitter.captures(remains).unwrap();
        let l: String = lr.get(1).unwrap().as_str().to_owned();
        let r: String = lr.get(2).unwrap().as_str().to_owned();

        // if !path.contains_key(&String::from(node_name)) {
            let l_pos: usize;
            let r_pos: usize;
            if !path.contains_key(&l) {
                if l != node_name {
                    graph.push(Node { name: l.clone(), left: usize::MAX , right: MAX });
                    l_pos = graph.len() - 1;
                    path.insert(l.clone(), ("".to_string(), "".to_string(), l_pos));
                } else {
                    l_pos = graph.len();
                }
            } else {
                l_pos = path.get(&l).unwrap().2;
            }
            if !path.contains_key(&r) {
                if r != node_name {
                    graph.push(Node { name: r.clone(), left: usize::MAX , right: MAX });
                    r_pos = graph.len() - 1;
                    path.insert(r.clone(), ("".to_string(), "".to_string(), r_pos));
                } else {
                    r_pos = graph.len();
                }
            } else {
                r_pos = path.get(&r).unwrap().2;
            }

            if !path.contains_key(&String::from(node_name)) {
                graph.push(Node { name: String::from(node_name), left: l_pos , right: r_pos });
                path.insert(String::from(node_name), (l, r, graph.len() - 1));
            } else {
                let in_path: &(String, String, usize) = path.get(&String::from(node_name)).unwrap();
                let node: &mut Node = &mut graph[in_path.2];
                node.left = l_pos;
                node.right = r_pos;
            }

        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    let mut current_dir:usize;
    // current_dir = 0;
    // let mut steps: u32 = 0;
    // let mut current_node_name = "AAA";
    // let first_node: &(String, String, _) = path.get("AAA").unwrap();
    // let mut pos = first_node.2;

    // while current_node_name != "ZZZ" {
    //     if current_dir == instructions.len() {
    //         current_dir = 1;
    //     } else {
    //         current_dir += 1;
    //     }
    //     if instructions[current_dir - 1] == 'L' {
    //         pos = graph[pos].left;
    //     } else {
    //         pos = graph[pos].right;
    //     }
    //     current_node_name = graph[pos].name.as_str();
    //     steps += 1;
    // }

    // println!("{}", steps);

    // part 2
    let mut max_values = usize::MAX;
    let mut positions: Vec<usize> = vec![];
    for (position, node) in graph.iter().enumerate() {
        if node.name.ends_with("A") {
            positions.push(position);
            max_values -= 1;
            if max_values == 0 {
                break;
            }
        }
    }
    // println!("{:?}", positions);
    // println!("{:?}", graph);

    let mut done = false;
    current_dir = 0;
    let mut all_steps: Vec<usize> = Vec::with_capacity(positions.len());
    for _ in &positions {
        all_steps.push(0);
    }

    while !done {
        if current_dir == instructions.len() {
            current_dir = 1;
        } else {
            current_dir += 1;
        }

        let old_positions = positions.clone();
        positions.clear();
        if instructions[current_dir - 1] == 'L' {
            for (old_pos, pos) in old_positions.iter().enumerate() {
                if !graph[*pos].name.ends_with("Z") {
                    positions.push(graph[*pos].left);
                    all_steps[old_pos] += 1;
                } else {
                    positions.push(*pos);
                }
            }
        } else {
            for (old_pos, pos) in old_positions.iter().enumerate() {
                if !graph[*pos].name.ends_with("Z") {
                    positions.push(graph[*pos].right);
                    all_steps[old_pos] += 1;
                } else {
                    positions.push(*pos);
                }
            }
        }
        
        done = true;
        for pos in &positions {
            if !graph[*pos].name.ends_with("Z") {
                done = false;
                break;
            }
        }

    }

    // println!("{:?}", all_steps);
    
    let mut result2 = all_steps[0];
    for pos in 1..all_steps.len() {
        // print!("{}  ", all_steps[pos]);
        result2 = lcm(result2, all_steps[pos]);
    }

    let elapsed = now.elapsed();

    println!("{} -- {}", result2, elapsed.as_millis());

    Ok(())
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp_a = a;
        a = b;
        b = tmp_a % b;
    }
    return a;
}
