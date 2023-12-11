use std::cmp::Ordering;

use crate::day_seven::d7_structs::{CardKind, Bid};
use crate::utils::file_reader::read_lines;

pub fn d7p1 () {
    let mut vec = build_up_bids();
    let _= vec.sort_by(compare_plays);   
    let result = calculate_winnings(&vec);
    println!("D7P1 result: {}", result);
}

fn calculate_winnings(bids:&Vec<Bid>) -> u32 {
    let mut result: u32 = 0;

    for i in 0..bids.len() {
        let current_bid = bids.get(i).unwrap(); 
        result += (i as u32+1) * current_bid.bid;
    }
    result
}

pub fn build_up_bids() -> Vec<Bid> {
    let mut bids:Vec<Bid> = Vec::new();

       if let Ok(lines) = read_lines("./resources/d7.txt") {
        for line in lines {
            if let Ok(entry) = line {
                let (card, bid) = get_cards_and_bid(&entry);
                let card_kind = determine_kind(card);
                 bids.push(Bid {bid, card: String::from(card), card_kind });
            }
        }
    }
    bids
}

fn get_cards_and_bid(entry: &str) -> (&str, u32){
    let parts = entry.split(" ");
    (parts.clone().nth(0).unwrap(), parts.clone().nth(1).unwrap().parse::<u32>().ok().unwrap())
}

fn determine_kind(bid: &str) -> CardKind {
    let mut arr:[u8;15] = [0;15];

    for card in bid.chars() {
        let abs_value = get_card_absolute_value(card);
        arr[abs_value]+=1;
    }


    let mut three_flag: bool = false;
    let mut two_flag: bool = false;


    for num in arr {
        match num {
            5 => return CardKind::FIOK,
            4 => return CardKind::FOOK,
            3 => {
                if two_flag {
                    return CardKind::FH;
                } else {
                    three_flag = true;
                }
            },
            2 => {
                if three_flag {
                    return  CardKind::FH;
                } else if two_flag {
                    return CardKind::TP;
                } else {
                    two_flag = true;
                }
            },
            _ => continue,
        }

    }

    if three_flag && !two_flag {
        return CardKind::TOK;
    }else if !three_flag && two_flag {
        return CardKind::OP;
    }
    CardKind::HC
}

fn compare_plays(a: &Bid, b: &Bid) -> Ordering {
    let a_value = get_kind_power(&a.card_kind);
    let b_value = get_kind_power(&b.card_kind);

    if a_value == b_value {
        let a_play = a.card.chars();
        let b_play = b.card.chars();

        for i in 0..5 {
            let a_abs_value = get_card_absolute_value(a_play.clone().nth(i).unwrap());
            let b_abs_value = get_card_absolute_value(b_play.clone().nth(i).unwrap());

            if a_abs_value == b_abs_value {
                continue;
            }

            return a_abs_value.cmp(&b_abs_value);
        }

    }

    a_value.cmp(&b_value)
}

fn get_card_absolute_value(card: char) -> usize {
    if card == '1' {
        return 1;
    } else if card == '2' {
        return 2;
    } else if card == '3' {
        return 3;
    } else if card == '4' {
        return 4;
    } else if card == '5' {
        return 5;
    } else if card == '6' {
        return 6;
    } else if card == '7' {
        return 7;
    } else if card == '8' {
        return 8;
    } else if card == '9' {
        return 9;
    } else if card == 'T' {
        return 10;
    } else if card == 'J' {
        return 11;
    } else if card == 'Q' {
        return 12;
    } else if card == 'K' {
        return 13;
    } else if card == 'A' {
        return 14;
    } 
    0
}


fn get_kind_power(kind: &CardKind) -> usize {
    match kind {
        CardKind::FIOK => 7,
        CardKind::FOOK => 6,
        CardKind::FH => 5,
        CardKind::TOK => 4,
        CardKind::TP => 3,
        CardKind::OP => 2,
        CardKind::HC => 1,
    }
}
