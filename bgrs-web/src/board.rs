extern crate bgrs_logic;

use bgrs_logic::{BoardState, PlayerColor, PointState};
use yew::prelude::*;

use super::bar::Bar;
use super::point::{Point, PointDirection};

#[derive(Clone, PartialEq)]
pub struct BoardProps {
    width: i32,
    height: i32,
    board: BoardState,
}

impl Default for BoardProps {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 600,
            board: BoardState::new_starting_state(PlayerColor::Black),
        }
    }
}

pub struct Board {
    props: BoardProps,
}

impl Component for Board {
    type Message = ();
    type Properties = BoardProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Board { props }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<Board> for Board {
    fn view(&self) -> Html<Self> {
        let BoardProps {
            width,
            height,
            ref board,
        } = self.props;
        html! {
            <svg width=width, height=height,>
                <rect
                    class="board-bg",
                    width=width,
                    height=height,
                />

                { for (&board.points[1..=24]).iter().enumerate().map(
                    |(i, p)| self.render_point(i as i32, p)
                )}

                <Bar:
                    x=self.point_x_index_to_x(7),
                    width=self.point_width(),
                    height=height,
                    black_count=board.points[0].checker_count,
                    white_count=board.points[25].checker_count,
                />
            </svg>
        }
    }
}

impl Board {
    fn point_x_margin(&self) -> i32 {
        10
    }

    fn point_y_margin(&self) -> i32 {
        45
    }

    fn point_width(&self) -> i32 {
        (self.props.width - (11 * self.point_x_margin())) / 15
    }

    fn point_height(&self) -> i32 {
        (self.props.height - self.point_y_margin()) / 2
    }

    fn point_x_index_to_x(&self, point_x_index: i32) -> i32 {
        (self.point_width() + self.point_x_margin()) * point_x_index
    }

    fn point_y_index_to_y(&self, point_y_index: i32) -> i32 {
        (self.point_height() + self.point_y_margin()) * point_y_index
    }

    fn render_point(&self, index: i32, point: &PointState) -> Html<Self> {
        let (point_x_index, point_y_index, dir) = if index < 12 {
            (11 - index, 1, PointDirection::Up)
        } else {
            (index - 12, 0, PointDirection::Down)
        };

        // make room for bar and "end" points
        let point_x_index = if point_x_index < 6 {
            point_x_index + 1
        } else {
            point_x_index + 2
        };

        html! {
            <Point:
                is_odd=(index % 2) == 1,
                x=self.point_x_index_to_x(point_x_index),
                y=self.point_y_index_to_y(point_y_index),
                width=self.point_width(),
                height=self.point_height(),
                dir=dir,
                state=*point,
            />
        }
    }
}
