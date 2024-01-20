use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}, usize };

fn string_to_hashset(str_num: &str) -> HashSet<u32> {
    let mut ret: HashSet<u32> = HashSet::new();

    let num_strings: Vec<&str> = str_num.split(' ').into_iter().collect();

    for num in num_strings.iter() {
        if *num == "" {
            continue;
        }
        let n = num.parse().unwrap();
        ret.insert(n);
    }

    ret
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;

    let mut reader = BufReader::new(file);

    let mut result: i32 = 0;
    let mut cards: HashMap<u32, u32> = HashMap::new();

    let mut line: String = String::new();
    let mut linelen = reader.read_line(&mut line)?;
    while linelen != 0 {
        line = String::from(line.trim_end());

        let card_details:Vec<&str> = line.split(": ").collect();

        let (_, card_info) = card_details[0].split_at(4);

        let card_number: u32 = card_info.trim().parse().unwrap_or(9999);
        if card_number == 9999 {
            println!("{:?} --- {:?}", card_info, line);
        }


        let all_card_numbers:Vec<&str> = card_details[1].split(" | ").into_iter().collect();

        let win_numbers = string_to_hashset(all_card_numbers[0]);
        let card_numbers = string_to_hashset(all_card_numbers[1]);

        let intersection:Vec<_> = win_numbers.intersection(&card_numbers).collect();

        let count = intersection.len() as u32;
        cards.insert(card_number, count);

        if count  > 0 {
            result += i32::pow(2, count - 1);
        }

        line = String::from("");
        linelen = reader.read_line(&mut line)?;
    }

    let mut cards_counter: Vec<u32> = Vec::with_capacity(cards.len());

    for (_, _) in cards.iter() {
        cards_counter.push(1);
    }

    let mut idx = 0;

    while idx < cards_counter.len() {
        let num_cards = cards_counter[idx];
        let base_num_cards = cards.get(&((idx + 1) as u32)).unwrap();
        for stepper in idx + 1..idx + *base_num_cards as usize + 1 {
            cards_counter[stepper as usize] = cards_counter[stepper as usize] + num_cards;
        } 

        idx += 1;
    }

    let mut result2: u32 = 0;
    for value in cards_counter {
        result2 += value;
    }

    println!("{}", result);
    println!("{}", result2);

    Ok(())
}
