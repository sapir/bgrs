extern crate bgrs_logic;

use bgrs_logic::PlayerColor;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CheckerProps {
    pub x: i32,
    pub y: i32,
    pub d: u32,
    pub color: PlayerColor,
}

impl Default for CheckerProps {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            d: 80,
            color: PlayerColor::Black,
        }
    }
}

pub struct Checker {
    props: CheckerProps,
}

impl Component for Checker {
    type Message = ();
    type Properties = CheckerProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Checker { props }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<Checker> for Checker {
    fn view(&self) -> Html<Self> {
        let CheckerProps { x, y, d, color } = self.props;

        let r = d / 2;

        html! {
            <circle
                class=format!(
                    "checker {}",
                    match color {
                        PlayerColor::Black => "black-player",
                        PlayerColor::White => "white-player",
                    },
                ),
                cx=x,
                cy=y,
                r=r,
            />
        }
    }
}
