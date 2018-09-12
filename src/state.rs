#[derive(Clone, Copy, Debug)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor {
    pub fn inverse(&self) -> Self {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointState {
    checker_count: usize,
    checker_color: PlayerColor,
}

impl PointState {
    fn new(checker_count: usize, checker_color: PlayerColor) -> Self {
        Self {
            checker_count,
            checker_color,
        }
    }
}

type PointIndex = usize;

#[derive(Clone, Debug)]
pub struct BoardState([PointState; 26]);

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
            let opposite = points[BoardState::get_opposite(i)];
            points[i] = PointState::new(
                opposite.checker_count,
                opposite.checker_color.inverse(),
            );
        }
        BoardState(points)
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
                        "*"
                    } else {
                        BoardState::get_checker_string(point.checker_color)
                    }
                } else {
                    "|"
                }
            );
        }
    }

    pub fn print(&self) {
        for row in 0..6 {
            self.print_points(row, (&self.0[13..19]).iter());

            print!(
                " {} ",
                if row >= 6 - self.0[25].checker_count {
                    BoardState::get_checker_string(
                        self.0[25].checker_color,
                    )
                } else {
                    "*"
                }
            );

            self.print_points(row, (&self.0[19..25]).iter());
            println!();
        }

        println!();

        for row in 0..6 {
            let row = 5 - row;
            self.print_points(row, self.0[7..13].iter().rev());

            print!(
                " {} ",
                if row >= 6 - self.0[0].checker_count {
                    BoardState::get_checker_string(self.0[0].checker_color)
                } else {
                    "*"
                }
            );

            self.print_points(row, self.0[1..7].iter().rev());
            println!();
        }
    }
}
