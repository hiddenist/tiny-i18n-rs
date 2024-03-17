use crate::components::app::App;

mod components;
mod lang;

fn main() {
    yew::Renderer::<App>::new().render();
}
