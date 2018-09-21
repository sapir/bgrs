use bgrs_logic::PlayerColor;
use yew::prelude::*;

use super::checker_group::{CheckerGroup, CheckerGroupVAlign};
use super::svg::translate;

#[derive(Clone, PartialEq, Default)]
pub struct BarProps {
    pub x: i32,
    pub y: i32,
    pub black_count: usize,
    pub white_count: usize,
}

pub struct Bar {
    props: BarProps,
}

impl Component for Bar {
    type Message = ();
    type Properties = BarProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Bar { props }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<Bar> for Bar {
    fn view(&self) -> Html<Self> {
        let BarProps {
            x,
            y,
            black_count,
            white_count,
        } = self.props;

        let has_both = black_count > 0 && white_count > 0;

        html! {
            <g transform=translate(x, y),>
                <CheckerGroup:
                    valign=if has_both { CheckerGroupVAlign::Bottom } else { CheckerGroupVAlign::Center },
                    count=black_count,
                    color=PlayerColor::Black,
                />

                <CheckerGroup:
                    valign=if has_both { CheckerGroupVAlign::Top } else { CheckerGroupVAlign::Center },
                    count=white_count,
                    color=PlayerColor::White,
                />
            </g>
        }
    }
}
