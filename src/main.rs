extern crate dedup_iter;
extern crate rand;

mod state;

use dedup_iter::DedupAdapter;
use rand::Rng;
use state::{BoardState, Move};
use std::io::{self, Write};

type DieRoll = usize;

fn roll_die() -> DieRoll {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 7)
}

type DiceRoll = (DieRoll, DieRoll);

fn roll_dice() -> DiceRoll {
    (roll_die(), roll_die())
}

fn uniq_map<I, T, F, R>(iterator: I, f: F) -> Vec<R>
where
    I: Iterator<Item = T>,
    F: FnMut(T) -> R,
    R: Ord,
{
    let mut ret: Vec<R> = iterator.map(f).collect();
    ret.sort();
    ret.dedup();
    ret
}

fn get_num_input(prompt: &str) -> io::Result<usize> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
        print!("{}", prompt);
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        if let Ok(num) = line.trim().parse() {
            return Ok(num);
        }
    }
}

fn fmt_array<T>(indices: &[T]) -> String
where
    T: ToString,
{
    indices
        .iter()
        .map(T::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}

// returns player's valid move, or None if there isn't any
fn get_human_player_move_seq(
    board: &BoardState,
    dice: DiceRoll,
) -> io::Result<Option<Vec<Move>>> {
    let valid_move_seqs = board.get_move_seqs(dice);
    if valid_move_seqs.is_empty() {
        return Ok(None);
    }

    // all valid_move_seqs are the same length, as player must make the maximum
    // number of moves available. so we can just check the length of the first
    // sequence.
    let expected_length = valid_move_seqs[0].len();

    let mut cur_board = board.clone();
    let mut ret = Vec::new();
    while ret.len() < expected_length {
        cur_board.print();

        let valid_next_moves: Vec<Move> = valid_move_seqs
            .iter()
            .filter(|&s| s[..(ret.len())] == ret[..])
            .map(|s| s[ret.len()])
            // iterator generation basically makes it sorted, so this removes
            // all duplicates
            .dedup()
            .collect();

        let valid_start_points = uniq_map(valid_next_moves.iter(), |m| m.0);

        let start_point = get_num_input(&format!(
            "start point? ({}{}): ",
            fmt_array(&valid_start_points),
            if !ret.is_empty() { "; 0 to undo)" } else { "" },
        ))?;

        if start_point == 0 {
            ret.pop();
            cur_board = board.with_move_seq(ret.iter());
            continue;
        }

        let valid_distances = uniq_map(
            valid_next_moves.iter().filter(|m| m.0 == start_point),
            |m| m.1 - start_point,
        );

        let distance = get_num_input(&format!(
            "distance? ({}; 0 to undo): ",
            fmt_array(&valid_distances),
        ))?;

        if distance == 0 {
            // undo start point; don't modify ret
            continue;
        }

        let move_ = Move(start_point, start_point + distance);
        ret.push(move_);
        cur_board = cur_board.with_move(move_);
    }

    Ok(Some(ret))
}

fn main() {
    let mut board = BoardState::new();

    let dice = roll_dice();
    println!("Dice: {:?}", dice);

    let move_seq = get_human_player_move_seq(&board, dice);
    println!("chosen seq: {:?}", move_seq);
}
