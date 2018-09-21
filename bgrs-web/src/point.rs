extern crate bgrs_logic;

use bgrs_logic::{PlayerColor, PointState};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PointDirection {
    Up,
    Down,
}

#[derive(Clone, PartialEq)]
pub struct PointProps {
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
            x,
            y,
            width,
            height,
            dir,
            ref state,
        } = self.props;

        html! {
            <path
                class="point",
                d=format!(
                    "M{} {} h{} l{} {} Z",
                    x,
                    match dir {
                        PointDirection::Up => y + height,
                        PointDirection::Down => y,
                    },
                    width,
                    -width / 2,
                    match dir {
                        PointDirection::Up => -height,
                        PointDirection::Down => height,
                    }),
            />
        }
    }
}
