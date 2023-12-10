mod utils;
mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;

use day_one::d1p1;
use day_one::d1p2;

use day_two::d2p1;
use day_two::d2p2;


use day_three::d3p1;
use day_three::d3p2;

use day_four::d4p1;
use day_four::d4p2;

use day_five::d5p1;
use day_five::d5p2;

fn main() {
    d1p1::d1p1();
    d1p2::d1p2();
    d2p1::d2p1();
    d2p2::d2p2();
    d3p1::d3p1();
    d3p2::d3p2();
    d4p1::d4p1();
    d4p2::d4p2();
    d5p1::d5p1();
    d5p2::d5p2();
}
