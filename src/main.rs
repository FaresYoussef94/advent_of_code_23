mod utils;
mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eigth;
mod day_nine;
mod day_fifteen;

use clap::Parser;
use utils::challenge_init::call_challenge;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
}

fn main() {

    let args = Args::parse();
    call_challenge(args.day, args.part);
}
