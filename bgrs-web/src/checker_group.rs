extern crate bgrs_logic;

use bgrs_logic::PlayerColor;
use std::cmp::min;
use yew::prelude::*;

use super::checker::Checker;
use super::svg::translate;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckerGroupVAlign {
    Top,
    Bottom,
    Center,
}

#[derive(Clone, PartialEq)]
pub struct CheckerGroupProps {
    pub x: i32,
    pub y: i32,
    pub max_checkers: usize,
    pub valign: CheckerGroupVAlign,
    pub count: usize,
    pub color: PlayerColor,
}

impl Default for CheckerGroupProps {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            max_checkers: 5,
            valign: CheckerGroupVAlign::Top,
            count: 1,
            color: PlayerColor::Black,
        }
    }
}

pub struct CheckerGroup {
    props: CheckerGroupProps,
}

impl Component for CheckerGroup {
    type Message = ();
    type Properties = CheckerGroupProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CheckerGroup { props }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<CheckerGroup> for CheckerGroup {
    fn view(&self) -> Html<Self> {
        let CheckerGroupProps {
            x,
            y,
            max_checkers,
            count,
            valign,
            color,
        } = self.props;

        // TODO: props
        let checker_d = 50;
        let checker_margin = 5;

        let displayed = min(max_checkers, count);

        let height = if displayed == 0 {
            0 as i32
        } else {
            ((displayed - 1) as i32) * checker_margin
                + (displayed as i32) * checker_d
        };

        let top = match valign {
            CheckerGroupVAlign::Top => y,
            CheckerGroupVAlign::Center => y - height / 2,
            CheckerGroupVAlign::Bottom => y - height,
        };

        html! {
            <g transform=translate(x, top),>
                { for (0..displayed).into_iter().map(
                    |i| html! {
                        <Checker:
                            y=(
                                (checker_d + checker_margin) * (i as i32)
                                + checker_d / 2
                            ),
                            d=checker_d as u32,
                            color=color,
                        />
                    }
                )}
            </g>
        }
    }
}
