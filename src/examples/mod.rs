#![allow(dead_code)]
#![allow(unused_variables)]

mod guess_game;

pub fn run_examples() {
    guess_game::play_guess_number_game();

    println!("end of examples")
}
