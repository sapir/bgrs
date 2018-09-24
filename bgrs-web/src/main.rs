#![recursion_limit = "1024"]

#[macro_use]
extern crate yew;
use yew::prelude::*;

extern crate bgrs_logic;

mod bar;
mod board;
mod checker;
mod checker_group;
mod point;
mod svg;

use self::board::Board;
use bgrs_logic::{BoardState, PlayerColor};

struct Game {
    state: BoardState,
}

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Game {
            state: BoardState::new_starting_state(PlayerColor::Black),
        }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<Game> for Game {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Board: board=self.state.clone(), />
            </>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Game>::new().mount_to_body();
    yew::run_loop();
}
