use std::{fs::File, io::{BufReader, BufRead}, collections:: HashMap, cmp::Ordering };

#[derive(PartialOrd, PartialEq, Eq)]
#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: u16,
    bid: u32,
}

impl Hand {
    fn calculate_type(&mut self) {
        let mut buckets: HashMap<char, u16> = HashMap::new();
        let mut count: u16;
        let cards_char:Vec<char> = self.cards.chars().collect();
        for card in cards_char.iter() {
            if buckets.contains_key(&card) {
                count = *buckets.get(&card).unwrap();
                buckets.insert(*card, count + 1);
            } else {
                buckets.insert(*card, 1);
            }
        }

        match buckets.len() {
            1 => self.hand_type = 7, // five of a kind
            2 => {
                let first_card = cards_char[0];
                    let num_first_card = *buckets.get(&first_card).unwrap();
                    if num_first_card == 1 || num_first_card == 4 {
                        self.hand_type = 6; // four of a kind
                    } else {
                        self.hand_type = 5; // full house
                    }
                }
            3 => {
                    let mut max_val: u16 = 0;
                    for (_, val) in buckets.iter() {
                        if max_val < *val {
                            max_val = *val;
                            if max_val == 3 {
                                break;
                            }
                        }
                    }
                    if max_val == 3 {
                        self.hand_type = 4; // three of a kind
                    } else {
                        self.hand_type = 3; // two pairs
                    }

                }
            4 => self.hand_type = 2, // one pair
            5 => self.hand_type = 1, // high card
            _ => panic!(),
        }
    }

    fn calculate_type2(&mut self, card_values: &HashMap<&char, &u16>) {
        let mut buckets: HashMap<char, u16> = HashMap::new();
        let mut count: u16;
        let mut cards_char:Vec<char> = self.cards.chars().collect();
        for card in cards_char.iter() {
            if buckets.contains_key(&card) {
                count = *buckets.get(&card).unwrap();
                buckets.insert(*card, count + 1);
            } else {
                buckets.insert(*card, 1);
            }
        }

        print!("before: {:?}", cards_char);

        if buckets.contains_key(&'J') {
            let j_count = buckets.get(&'J').cloned().unwrap();
            let mut max_char = 'J';
            let mut max: u16 = 0;
            for k in buckets.keys() {
                let current = buckets.get(k).unwrap();
                if *k != 'J' && *current > max {
                    max_char = *k;
                    max = *current;
                } else if *k != 'J' && *current == max && card_values.get(k).unwrap() > card_values.get(&max_char).unwrap() {
                    max_char = *k;
                    max = *current;
                }
            }
            buckets.remove(&'J');
            buckets.insert(max_char, max + j_count);
            for pos in 0..5 {
                if cards_char[pos] == 'J' {
                    cards_char[pos] = max_char;
                }
            }
            print!(" --- after: {:?}", cards_char);
        }

        match buckets.len() {
            1 => self.hand_type = 7, // five of a kind
            2 => {
                let first_card = cards_char[0];
                    let num_first_card = *buckets.get(&first_card).unwrap();
                    if num_first_card == 1 || num_first_card == 4 {
                        self.hand_type = 6; // four of a kind
                    } else {
                        self.hand_type = 5; // full house
                    }
                }
            3 => {
                    let mut max_val: u16 = 0;
                    for (_, val) in buckets.iter() {
                        if max_val < *val {
                            max_val = *val;
                            if max_val == 3 {
                                break;
                            }
                        }
                    }
                    if max_val == 3 {
                        self.hand_type = 4; // three of a kind
                    } else {
                        self.hand_type = 3; // two pairs
                    }

                }
            4 => self.hand_type = 2, // one pair
            5 => self.hand_type = 1, // high card
            _ => panic!(),
        }
        println!("--- {:?}", self);
    }

    fn cmp(&self, other: &Self, card_values: &HashMap<&char, &u16>) -> Ordering {
        let mut ret: Ordering = Ordering::Equal;
        // println!("{} - {}", self.cards, other.cards);
        if self.hand_type < other.hand_type {
            ret = Ordering::Less;
        } else if self.hand_type > other.hand_type {
            ret = Ordering::Greater;
        } else {
            let self_cards_char:Vec<char> = self.cards.chars().collect();
            let other_cards_char:Vec<char> = other.cards.chars().collect();
            for pos in 0..5 {
                let self_card_value: &&u16 = card_values.get(&self_cards_char[pos]).unwrap();
                let other_card_value: &&u16 = card_values.get(&other_cards_char[pos]).unwrap();

                // println!("{}, {} - {}, {}", self_cards_char[pos], self_card_value, other_cards_char[pos], other_card_value);

                if self_card_value < other_card_value {
                    ret = Ordering:: Less;
                    break;
                } else if self_card_value > other_card_value {
                    ret = Ordering::Greater;
                    break;
                }
            }
        }

        // println!("{:?}", ret);

        ret
    }
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut linelen: usize = reader.read_line(&mut line)?;

    let mut card_values: HashMap<&char, &u16> = HashMap::new();
    card_values.insert(&'2', &1);
    card_values.insert(&'3', &2);
    card_values.insert(&'4', &3);
    card_values.insert(&'5', &4);
    card_values.insert(&'6', &5);
    card_values.insert(&'7', &6);
    card_values.insert(&'8', &7);
    card_values.insert(&'9', &8);
    card_values.insert(&'T', &9);
    card_values.insert(&'J', &10);
    card_values.insert(&'Q', &11);
    card_values.insert(&'K', &12);
    card_values.insert(&'A', &13);

    let mut hands: Vec<Hand> = vec![];

    while linelen != 0 {
        let trim_line: &str = line.trim_end();

        let cards_bid: (&str, &str) = trim_line.split_at(5);

        let bid: u32 = cards_bid.1.trim_start().parse::<u32>().unwrap();

        let mut hand: Hand = Hand{ cards: String::from(cards_bid.0), bid, hand_type: 0 };
        hand.calculate_type();

        hands.push(hand);

        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    hands.sort_by(| a: &Hand, b: &Hand | a.cmp(b, &card_values));

    // println!("{:?}", hands);

    let mut result1: u32 = 0;
    let mut pos: u32 = 0;

    for hand in hands {
        pos += 1;
        result1 += hand.bid * pos;
    }
    println!("{:?}", result1);

    // part 2
    let file: File = File::open("input")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();
    let mut linelen: usize = reader.read_line(&mut line)?;

    let mut card_values2: HashMap<&char, &u16> = HashMap::new();
    card_values2.insert(&'J', &1);
    card_values2.insert(&'2', &2);
    card_values2.insert(&'3', &3);
    card_values2.insert(&'4', &4);
    card_values2.insert(&'5', &5);
    card_values2.insert(&'6', &6);
    card_values2.insert(&'7', &7);
    card_values2.insert(&'8', &8);
    card_values2.insert(&'9', &9);
    card_values2.insert(&'T', &10);
    card_values2.insert(&'Q', &11);
    card_values2.insert(&'K', &12);
    card_values2.insert(&'A', &13);


    let mut hands: Vec<Hand> = vec![];

    while linelen != 0 {
        let trim_line: &str = line.trim_end();

        let cards_bid: (&str, &str) = trim_line.split_at(5);

        let bid: u32 = cards_bid.1.trim_start().parse::<u32>().unwrap();

        let mut hand: Hand = Hand{ cards: String::from(cards_bid.0), bid, hand_type: 0 };
        hand.calculate_type2(&card_values2);

        hands.push(hand);

        line.clear();
        linelen = reader.read_line(&mut line)?;
    }

    hands.sort_by(| a: &Hand, b: &Hand | a.cmp(b, &card_values2));
    println!("{:?}", hands);

    let mut result2: u32 = 0;
    let mut pos: u32 = 0;

    for hand in hands {
        pos += 1;
        result2 += hand.bid * pos;
    }
    
    println!("{:?}", result2);

    Ok(())
}
