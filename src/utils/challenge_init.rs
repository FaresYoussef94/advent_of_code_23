use crate::day_one::d1p1::d1p1;
use crate::day_one::d1p2::d1p2;

use crate::day_two::d2p1::d2p1;
use crate::day_two::d2p2::d2p2;

use crate::day_three::d3p1::d3p1;
use crate::day_three::d3p2::d3p2;

use crate::day_four::d4p1::d4p1;
use crate::day_four::d4p2::d4p2;

use crate::day_five::d5p1::d5p1;
use crate::day_five::d5p2::d5p2;

use crate::day_six::d6p1::d6p1;
use crate::day_six::d6p2::d6p2;

use crate::day_seven::d7p1::d7p1;
use crate::day_seven::d7p2::d7p2;

use crate::day_eigth::d8p1::d8p1;
use crate::day_eigth::d8p2::d8p2;

use crate::day_nine::d9p1::d9p1;
use crate::day_nine::d9p2::d9p2;

use crate::day_ten::d10p1::d10p1;
use crate::day_ten::d10p2::d10p2;

use crate::day_fifteen::d15p1::d15p1;
use crate::day_fifteen::d15p2::d15p2;

use crate::day_seventeen::d17p1::d17p1;
use crate::day_seventeen::d17p2::d17p2;

pub fn call_challenge(day:u8, part:u8) {
    match day { 
        1 => call_part(part, d1p1, d1p2),
        2 => call_part(part, d2p1, d2p2),
        3 => call_part(part, d3p1, d3p2),
        4 => call_part(part, d4p1, d4p2),
        5 => call_part(part, d5p1, d5p2),
        6 => call_part(part, d6p1, d6p2),
        7 => call_part(part, d7p1, d7p2),
        8 => call_part(part, d8p1, d8p2),
        9 => call_part(part, d9p1, d9p2),
        10 => call_part(part, d10p1, d10p2),
        15 => call_part(part, d15p1, d15p2),
        17 => call_part(part, d17p1, d17p2),
        _ => unimplemented!("The following day wasn't solved yet or input is not corrupt") 
        
    }
}

fn call_part(part: u8, part_one: fn(), part_two:fn()) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => unimplemented!("Only part 1 and 2 exits") 
    }

}
