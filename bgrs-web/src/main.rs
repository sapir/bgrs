#![recursion_limit = "1024"]

#[macro_use]
extern crate yew;
use yew::prelude::*;

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
                <div>{"Some text"}</div>
                <div>{"More text"}</div>
            </>
        }
    }
}
fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
