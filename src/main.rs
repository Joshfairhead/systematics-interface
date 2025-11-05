mod app;
mod components;
mod core;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
