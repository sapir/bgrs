use std::cmp::max;
use std::collections::VecDeque;
use std::iter::repeat;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor {
    pub fn inverse(self) -> Self {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointState {
    pub checker_count: usize,
    pub checker_color: PlayerColor,
}

impl PointState {
    fn new(checker_count: usize, checker_color: PlayerColor) -> Self {
        Self {
            checker_count,
            checker_color,
        }
    }

    pub fn is_used_by(&self, player: PlayerColor) -> bool {
        self.checker_color == player && self.checker_count > 0
    }
}

type PointIndex = usize;

#[derive(Clone, Copy, Debug)]
pub struct Move(PointIndex, PointIndex);

impl Move {
    // get die roll used to make this move
    pub fn die_roll(&self) -> usize {
        ((self.1 as isize) - (self.0 as isize)).abs() as usize
    }
}

#[derive(Clone, Debug)]
pub struct BoardState {
    pub points: [PointState; 26],
    pub cur_player: PlayerColor,
}

impl BoardState {
    fn get_opposite(index: PointIndex) -> PointIndex {
        25 - index
    }

    pub fn new() -> Self {
        let mut points = [PointState::new(0, PlayerColor::Black); 26];
        points[1] = PointState::new(2, PlayerColor::Black);
        points[6] = PointState::new(5, PlayerColor::White);
        points[8] = PointState::new(3, PlayerColor::White);
        points[12] = PointState::new(5, PlayerColor::Black);
        for i in 13..26 {
            // make a copy of point to work around borrow checker
            let opposite = points[Self::get_opposite(i)];
            points[i] = PointState::new(
                opposite.checker_count,
                opposite.checker_color.inverse(),
            );
        }

        BoardState {
            points,
            cur_player: PlayerColor::Black,
        }
    }

    fn get_checker_string(color: PlayerColor) -> &'static str {
        match color {
            PlayerColor::Black => "b",
            PlayerColor::White => "w",
        }
    }

    fn print_points<'a, I>(&self, row_count: usize, points: I)
    where
        I: Iterator<Item = &'a PointState>,
    {
        let mut first: bool = true;

        for point in points {
            if first {
                first = false;
            } else {
                print!(" ");
            }

            print!(
                "{}",
                if point.checker_count > row_count {
                    if row_count >= 5 {
                        "+"
                    } else {
                        Self::get_checker_string(point.checker_color)
                    }
                } else {
                    "|"
                }
            );
        }
    }

    pub fn print(&self) {
        for row in 0..6 {
            self.print_points(row, (&self.points[13..19]).iter());

            print!(
                " {} ",
                if row >= 6 - self.points[25].checker_count {
                    Self::get_checker_string(self.points[25].checker_color)
                } else {
                    "*"
                }
            );

            self.print_points(row, (&self.points[19..25]).iter());
            println!();
        }

        println!();

        for row in 0..6 {
            let row = 5 - row;
            self.print_points(row, self.points[7..13].iter().rev());

            print!(
                " {} ",
                if row >= 6 - self.points[0].checker_count {
                    Self::get_checker_string(self.points[0].checker_color)
                } else {
                    "*"
                }
            );

            self.print_points(row, self.points[1..7].iter().rev());
            println!();
        }
    }

    pub fn used_points(&self, player: PlayerColor) -> Vec<PointIndex> {
        self.points
            .iter()
            .enumerate()
            .filter(|(_i, ps)| ps.is_used_by(player))
            .map(|(i, _)| i)
            .collect()
    }

    // swap point indices if player is white, so that 1 is start point and 24
    // is end point
    fn reverse_white_points(
        player: PlayerColor,
        index_vec: &mut Vec<PointIndex>,
    ) {
        if player == PlayerColor::White {
            index_vec
                .iter_mut()
                .for_each(|i| *i = Self::get_opposite(*i));
        }
    }

    // like reverse_white_point() for a Vec<Move>
    fn reverse_white_moves(player: PlayerColor, move_vec: &mut Vec<Move>) {
        if player == PlayerColor::White {
            move_vec.iter_mut().for_each(|m| {
                m.0 = Self::get_opposite(m.0);
                m.1 = Self::get_opposite(m.1);
            });
        }
    }

    // ignores effects of other die
    pub fn get_moves_for_single_die(&self, die_roll: usize) -> Vec<Move> {
        // must enter checkers on bar if possible
        let cur_player_bar = Self::get_bar_point(self.cur_player);
        let mut relevant_points =
            if self.points[cur_player_bar].checker_count > 0 {
                vec![cur_player_bar]
            } else {
                self.used_points(self.cur_player)
            };

        if relevant_points.is_empty() {
            return vec![];
        }

        Self::reverse_white_points(self.cur_player, &mut relevant_points);

        let farthest = *relevant_points.iter().min().unwrap();

        let bearing_off = farthest >= 19;

        let moves = relevant_points
            .iter()
            .map(|i| Move(*i, i + die_roll))
            // destination must be empty or a blot, if on the board
            .filter(|Move(_i, j)|
                // off the board is ok at this point
                *j > 24
                // empty or used by current player is ok
                || !self.points[*j].is_used_by(self.cur_player.inverse())
                // else used by other player. if only 1 checker is present,
                // also ok.
                || self.points[*j].checker_count == 1);

        let mut moves = if bearing_off {
            // destination allowed to be off the board, BUT must be either
            // exact or else this is the farthest move
            moves
                .filter(|Move(i, j)| *j == 25 || *i == farthest)
                .collect()
        } else {
            // destination must be on board
            moves.filter(|Move(_i, j)| *j <= 24).collect()
        };

        // unswap indices before returning
        Self::reverse_white_moves(self.cur_player, &mut moves);
        moves
    }

    // get index of fake point used as bar for player's piece. this is set up
    // such that moves from bar is moving into the other player's home, using
    // usual math.
    pub fn get_bar_point(player: PlayerColor) -> PointIndex {
        match player {
            PlayerColor::Black => 0,
            PlayerColor::White => 25,
        }
    }

    pub fn is_bar_point(pi: PointIndex) -> bool {
        pi < 1 || pi > 24
    }

    pub fn apply_move(&mut self, Move(i, j): Move) {
        assert!(self.points[i].is_used_by(self.cur_player));
        self.points[i].checker_count -= 1;

        if Self::is_bar_point(j) {
            // just remove the checker and we're done
            return;
        }

        if self.points[j].is_used_by(self.cur_player.inverse()) {
            assert_eq!(self.points[j].checker_count, 1);

            let bar_index = Self::get_bar_point(self.points[j].checker_color);
            self.points[bar_index].checker_count += 1;

            // clear checker count so that increasing by one works later
            self.points[j].checker_count = 0;
        }

        self.points[j].checker_color = self.cur_player;
        self.points[j].checker_count += 1;
    }

    pub fn apply_move_seq<'a, T>(&mut self, move_seq: T)
    where
        T: Iterator<Item = &'a Move>,
    {
        for &move_ in move_seq {
            self.apply_move(move_);
        }
    }

    pub fn with_move(&self, move_: Move) -> Self {
        let mut ret = self.clone();
        ret.apply_move(move_);
        ret
    }

    pub fn with_move_seq<'a, T>(&self, move_seq: T) -> Self
    where
        T: Iterator<Item = &'a Move>,
    {
        let mut ret = self.clone();
        ret.apply_move_seq(move_seq);
        ret
    }

    // helper function for get_moves(). generates list of possible move
    // sequences for a given permutation of the dice.
    fn backtrack_die_moves(&self, dice_slice: &[usize]) -> Vec<VecDeque<Move>> {
        let first_moves = if dice_slice.is_empty() {
            vec![]
        } else {
            self.get_moves_for_single_die(dice_slice[0])
        };

        let rest = &dice_slice[1..];
        if rest.is_empty() {
            first_moves
                .iter()
                .map(|&move_| {
                    let mut v = VecDeque::new();
                    v.push_back(move_);
                    v
                }).collect()
        } else {
            // if first_moves is empty then we don't get any moves! which means
            // that we don't try the rest of the dice in the slice. BUT this
            // function is called for every possible permutation, so if we don't
            // return the right move here, it'll just be returned for the other
            // permutation where the ones that work are first.
            first_moves
                .iter()
                .flat_map(move |&move_| {
                    let state_after_move = self.with_move(move_);
                    let mut next_move_seqs =
                        state_after_move.backtrack_die_moves(&dice_slice[1..]);
                    for mut next_moves in next_move_seqs.iter_mut() {
                        next_moves.push_front(move_);
                    }

                    next_move_seqs
                }).collect()
        }
    }

    pub fn get_move_seqs(&self, dice_roll: (usize, usize)) -> Vec<Vec<Move>> {
        // convert to vec + double doubles + generate all permutations
        let is_double = dice_roll.0 == dice_roll.1;
        let perms = if is_double {
            vec![repeat(dice_roll.0).take(4).collect()]
        } else {
            vec![
                vec![dice_roll.0, dice_roll.1],
                vec![dice_roll.1, dice_roll.0],
            ]
        };

        let move_seqs: Vec<Vec<Move>> = perms
            .into_iter()
            .flat_map(|perm| {
                // convert to Vecs
                self.backtrack_die_moves(&perm)
                    .into_iter()
                    .map(|move_seq| move_seq.into_iter().collect::<Vec<Move>>())
            }).collect();

        // must perform maximum number of moves
        let max_seq_len = move_seqs.iter().map(|s| s.len()).max().unwrap_or(0);
        let only_longest =
            move_seqs.into_iter().filter(|s| s.len() >= max_seq_len);

        // prefer larger dice. this is relevant iff not a double and max_seq_len
        // == 1.
        if !is_double && max_seq_len == 1 {
            let larger_die = max(dice_roll.0, dice_roll.1);
            only_longest
                .filter(|s| s[0].die_roll() == larger_die)
                .collect()
        } else {
            only_longest.collect()
        }
    }
}
