use crate::day_two::d2_structs::BallCount;
use crate::utils::file_reader::read_lines;

const MAX_RED_COUNT:u32 = 12;
const MAX_GREEN_COUNT:u32 = 13;
const MAX_BLUE_COUNT:u32 = 14;

pub fn d2p1(){
    let mut result: u32 = 0;
    if let Ok(lines) = read_lines("./resources/d2p1.txt") {
        for line in lines{
            if let Ok(entry) = line {
                result += get_addable_game_id(entry);
            }
        }
    }

    println!("D2P1 result: {}", result);
}

fn get_addable_game_id(entry:String) -> u32{
    let game_definition = entry.split(":");
    let game_id:u32 = game_definition.clone().nth(0).unwrap().to_string().split(" ").nth(1).unwrap().to_string().parse::<u32>().unwrap();

    let rounds = game_definition.clone().nth(1);
    let ball_count = get_max_ball_count(rounds.unwrap().to_string());

    if ball_count.red_count <= MAX_RED_COUNT && ball_count.green_count <= MAX_GREEN_COUNT && ball_count.blue_count <= MAX_BLUE_COUNT {
        game_id
    }else{
        0
    }
} 


pub fn get_max_ball_count(total_rounds: String) -> BallCount{
    let mut red_count: u32 = 1;
    let mut blue_count: u32 = 1;
    let mut green_count: u32 = 1;

    for round in total_rounds.split(";"){
        for ball in round.split(","){
            let count = ball.trim().split(" ").nth(0).unwrap().to_string().parse::<u32>().unwrap();
            let colour = ball.trim().split(" ").nth(1).unwrap();
            
            match colour {
                "red" => {
                    if count > red_count {
                        red_count = count;
                    }
                },
                "green" => {
                    if count > green_count {
                        green_count = count;
                    }
                },
                "blue" => {
                    if count > blue_count {
                        blue_count = count;
                    }
                },
                _ => {},
            }

        }
    }

    return BallCount{red_count, blue_count, green_count}
}


