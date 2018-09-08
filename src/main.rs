mod state;

fn main() {
    let board = state::BoardState::new();
    board.print();
}
