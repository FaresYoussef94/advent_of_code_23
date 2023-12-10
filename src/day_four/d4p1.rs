use crate::utils::file_reader::read_lines;
use crate::day_four::d4_structs::GameDefinition;


pub fn d4p1() {
    let game_definitions = get_game_definitions();
    let result = calculate_results(&game_definitions);

    println!("D4P1 result: {}", result);
}

pub fn get_game_definitions() -> [GameDefinition; 202] {
    let mut game_definitions: [GameDefinition; 202] = [GameDefinition::default(); 202];
    let mut i: usize = 0;

    if let Ok(lines) = read_lines("./resources/d4p1.txt") {
        for line in lines {
            if let Ok(entry) = line {
                let game_definition = get_game_definition(&entry);
                game_definitions[i] = game_definition;
                i+=1;
            }
        }
    }

    game_definitions
}

pub fn get_game_definition(entry: &str) -> GameDefinition{
    let game = entry.split(":");
    let game_id = parse_game_id(game.clone().nth(0).unwrap().to_string());
    let round = game.clone().nth(1).unwrap().split("|");
    let mut winning_cards:[u32;10] = [0;10];
    let mut round_cards:[u32;25] = [0;25];

    let mut j: usize = 0;
    for winning_card in round.clone().nth(0).unwrap().split(" ") {
        match winning_card.parse::<u32>().ok(){
            Some(id) => {
                winning_cards[j] = id;
                j+=1;
            }, 
            None => continue, 
        };
    }

    let mut i: usize = 0;
    for round_card in round.clone().nth(1).unwrap().split(" ") {
        match round_card.parse::<u32>().ok(){
            Some(id) => {
                round_cards[i] = id;
                i+=1;
            }, 
            None => continue, 
        };
    }

    return GameDefinition{ game_id, winning_cards, round_cards}
}


fn calculate_results(game_definitons: &[GameDefinition;202]) -> u32{
    let mut result = 0;
    for game_definition in game_definitons {
        result += calculate_round_winning(game_definition);
    }
    result
}

fn calculate_round_winning(game_definition: &GameDefinition) -> u32 {
    let mut round_result = 0;
    for winning_number in game_definition.winning_cards {
        for round_number in game_definition.round_cards {
            if winning_number == round_number {
                if round_result == 0 {
                    round_result = 1;
                }else {
                    round_result *=2;
                }
            }

        }
    }
    if round_result == 0 {
    }
    round_result
}

fn parse_game_id(entry: String) -> usize {
    for part in entry.split(" ") {
        match part.parse::<usize>().ok(){
            Some(id) => return id, 
            None => continue, 
        };
    }
    0
}
