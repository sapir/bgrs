#![recursion_limit = "1024"]

#[macro_use]
extern crate yew;
use yew::prelude::*;

extern crate bgrs_logic;

mod board;
mod point;

use self::board::Board;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Board: />
            </>
        }
    }
}
fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
