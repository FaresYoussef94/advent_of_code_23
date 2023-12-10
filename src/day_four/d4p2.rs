use crate::d4p1::get_game_definitions;

use super::d4_structs::GameDefinition;

//game # -- 1   2   3   4   5   6
//game 1 -- 1   2   2   2   2   1
//game 2 -- 1   2   4   4   2   1
//game 3 -- 1   2   4   8   6   1
//game 4 -- 1   2   4   8  14   1 

pub fn d4p2() {
    let game_definitions = get_game_definitions();
    let result = update_winnings(&game_definitions);

    println!("D4P2 result: {}", result);
}

fn update_winnings(game_definitions: &[GameDefinition;202]) -> u32 {
    let mut scratch_cards:[usize;203] = [1;203];
    let mut result: u32 = 0;
    for game_definition in game_definitions {
        let round_matches = calculate_matches(game_definition);
        if round_matches == 0 {
            continue;
        }
        for i in game_definition.game_id+1..=game_definition.game_id+round_matches{
            scratch_cards[i]+=scratch_cards[game_definition.game_id];
        }
    }
    for i in 1..203 {
        result+=scratch_cards[i] as u32;
    }
    result
}

fn calculate_matches(game_definition: &GameDefinition) -> usize {
    let mut matches_count:usize = 0;
    for winning_number in game_definition.winning_cards {
        for round_number in game_definition.round_cards {
            if winning_number == round_number {
                matches_count +=1;
            }
        }
    }
    matches_count
}
