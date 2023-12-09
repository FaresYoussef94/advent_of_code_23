use crate::utils::file_reader::read_lines;
use crate::day_two::d2p1::get_max_ball_count;

pub fn d2p2(){
    let mut result: u32 = 0;
    if let Ok(lines) = read_lines("./resources/d2p1.txt") {
        for line in lines{
            if let Ok(entry) = line {
                result += get_ball_count_product(entry);
            }
        }
    }

    println!("D2P2 result: {}", result);
}


fn get_ball_count_product(entry: String) -> u32{
    let mut game_definition = entry.split(":");
    let rounds = game_definition.nth(1);
    let ball_count = get_max_ball_count(rounds.unwrap().to_string());

    ball_count.red_count * ball_count.green_count * ball_count.blue_count
}
