extern crate bgrs_logic;

use bgrs_logic::{BoardState, PlayerColor, PointState};
use yew::prelude::*;

use super::point::{Point, PointDirection};

#[derive(Clone, PartialEq)]
pub struct BoardProps {
    width: i32,
    height: i32,
    board: Box<BoardState>,
}

impl Default for BoardProps {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 600,
            board: Box::new(BoardState::new_starting_state(PlayerColor::Black)),
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
            </svg>
        }
    }
}

impl Board {
    fn render_point(&self, index: i32, point: &PointState) -> Html<Self> {
        let props = &self.props;
        let width = props.width;
        let height = props.height;

        let point_x_margin = 10;
        let point_y_margin = 45;

        let point_width = (width - (11 * point_x_margin)) / 15;
        let point_height = (height - point_y_margin) / 2;

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

        let x = (point_width + point_x_margin) * point_x_index;
        let y = (point_height + point_y_margin) * point_y_index;

        html! {
            <Point:
                is_odd=(index % 2) == 1,
                x=x,
                y=y,
                width=point_width,
                height=point_height,
                dir=dir,
                state=*point,
            />
        }
    }
}
