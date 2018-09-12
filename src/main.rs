extern crate rand;

mod state;

use rand::Rng;

fn roll_die() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 7)
}

fn roll_dice() -> (usize, usize) {
    (roll_die(), roll_die())
}

fn main() {
    let board = state::BoardState::new();
    board.print();

    let dice = roll_dice();
    println!("Dice: {:?}", dice);
}
