use std::{fs::File, io::Read};
use std::cmp;

// instead of a map of player moves to the corresponding score, we just subtract the byte values
// from our base value to get the score.
const MY_CHOICE_ASCII_SCORE_BASE : char = 'W'; // character before X in ascii table
const THEIR_CHOICE_ASCII_SCORE_BASE : char = '@'; // character before A in ascii table

fn main() {
    let filename = "input2.txt";
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(file)  => file,
        Err(error)  => panic!("{}", error),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error)  => panic!("{}", error),
    }

    let mut total_score :  u32 = 0;
    for line in contents.lines() {
        let me_char = line.chars().nth(2).unwrap();
        let them_char = line.chars().nth(0).unwrap();


        let their_move =  them_char as u8 - THEIR_CHOICE_ASCII_SCORE_BASE as u8;
        let my_move =  me_char as u8 - MY_CHOICE_ASCII_SCORE_BASE as u8;


        let round_score = fight_and_score(their_move, my_move);
        total_score += round_score as u32 + my_move as u32;

        println!("They:{} Me:{} Score:{}", get_human_readable(their_move), get_human_readable(my_move), round_score);

    }

    println!("Score: {}", total_score);
}

// Rock paper scissors. true means c1 won
// ROCK = 0
// PAPER = 1
// SCISSORS = 2
// mathy approach
// If both numbers are the same, its a draw
// If both numbers are consecutive, the bigger one wins
// If both numbers aren’t consecutive, the smaller one wins
//
// returns the winning number
fn fight_and_score(them: u8, me: u8) -> u8 {
    let winner : u8 = match (me as i8  - them as i8).abs() {
        0 => 100,
        1 => cmp::max(me, them) ,
        _ => cmp::min(me, them) ,
    };

    return match winner {
        100 => 3, // draw is worth 3 points
        w if w == me => 6,
        w if w == them => 0,
        _ => 255
    };
}

fn get_human_readable(num: u8) -> char {
    return match num {
        1 => 'R',
        2 => 'P',
        3 => 'S',
        _ => '!'
    }
}
