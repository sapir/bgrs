use bgrs_logic::PlayerColor;
use yew::prelude::*;

use super::checker_group::{CheckerGroup, CheckerGroupVAlign};
use super::svg::translate;

#[derive(Clone, PartialEq, Default)]
pub struct BarProps {
    pub x: i32,
    pub width: i32,
    pub height: i32,
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
            width,
            height,
            black_count,
            white_count,
        } = self.props;

        let has_both = black_count > 0 && white_count > 0;

        let bar_width = 0.7 * width as f32;

        html! {
            <g transform=translate(x + width / 2, 0),>
                <rect
                    class="bar-bg",
                    x=-bar_width / 2.0,
                    y=0,
                    width=bar_width,
                    height=height,
                />

                <g transform=translate(0, height / 2),>
                    <CheckerGroup:
                        valign=if has_both {
                            CheckerGroupVAlign::Bottom
                        } else {
                            CheckerGroupVAlign::Center
                        },
                        count=black_count,
                        color=PlayerColor::Black,
                    />

                    <CheckerGroup:
                        valign=if has_both {
                            CheckerGroupVAlign::Top
                        } else {
                            CheckerGroupVAlign::Center
                        },
                        count=white_count,
                        color=PlayerColor::White,
                    />
                </g>
            </g>
        }
    }
}
