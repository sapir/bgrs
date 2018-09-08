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
        Self { checker_count, checker_color }
    }
}

#[derive(Clone, Debug)]
pub struct BoardState([PointState; 24]);

impl BoardState {
    pub fn new() -> Self {
        let mut points = [PointState::new(0, PlayerColor::Black); 24];
        points[0] = PointState::new(2, PlayerColor::Black);
        points[5] = PointState::new(5, PlayerColor::White);
        points[7] = PointState::new(3, PlayerColor::White);
        points[11] = PointState::new(5, PlayerColor::Black);
        for i in 12..24 {
            let opposite = points[23 - i];
            points[i] = PointState::new(
                opposite.checker_count,
                opposite.checker_color.inverse());
        }
        BoardState(points)
    }

    fn print_points<'a, I>(&self, row_count: usize, points: I)
    where I: Iterator<Item=&'a PointState>
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
                        match point.checker_color {
                            PlayerColor::Black => "b",
                            PlayerColor::White => "w",
                        }
                    }
                } else {
                    "|"
                }
            );
        }
    }

    pub fn print(&self) {
        for row in 0..6 {
            self.print_points(row, (&self.0[12..18]).iter());
            print!(" * ");
            self.print_points(row, (&self.0[18..24]).iter());
            println!();
        }

        println!();

        for row in 0..6 {
            let row = 5 - row;
            self.print_points(row, self.0[6..12].iter().rev());
            print!(" * ");
            self.print_points(row, self.0[0..6].iter().rev());
            println!();
        }
    }
}
