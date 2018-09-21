extern crate bgrs_logic;

use bgrs_logic::{PlayerColor, PointState};
use yew::prelude::*;

use super::checker_group::{CheckerGroup, CheckerGroupVAlign};
use super::svg::translate;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PointDirection {
    Up,
    Down,
}

#[derive(Clone, PartialEq)]
pub struct PointProps {
    pub is_odd: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub dir: PointDirection,
    pub state: PointState,
}

impl Default for PointProps {
    fn default() -> Self {
        Self {
            is_odd: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            dir: PointDirection::Up,
            state: PointState::new(0, PlayerColor::Black),
        }
    }
}

pub struct Point {
    props: PointProps,
}

impl Component for Point {
    type Message = ();
    type Properties = PointProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Point { props }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<Point> for Point {
    fn view(&self) -> Html<Self> {
        let PointProps {
            is_odd,
            x,
            y,
            width,
            height,
            dir,
            ref state,
        } = self.props;

        let flat_side_y = match dir {
            PointDirection::Up => y + height,
            PointDirection::Down => y,
        };

        html! {
            <g transform=translate(x, flat_side_y),>
                <path
                    class=format!(
                        "point {}",
                        if is_odd { "odd" } else { "even" },
                    ),
                    d=format!(
                        "M0 0 h{} l{} {} Z",
                        width,
                        -width / 2,
                        match dir {
                            PointDirection::Up => -height,
                            PointDirection::Down => height,
                        }),
                />

                <CheckerGroup:
                    x=width / 2,
                    valign=match dir {
                        PointDirection::Up => CheckerGroupVAlign::Bottom,
                        PointDirection::Down => CheckerGroupVAlign::Top,
                    },
                    color=state.checker_color,
                    count=state.checker_count,
                />
            </g>
        }
    }
}
