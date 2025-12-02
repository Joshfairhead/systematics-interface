mod app;
mod api_app;
mod components;
mod core;
mod api;

use api_app::ApiApp;

fn main() {
    yew::Renderer::<ApiApp>::new().render();
}
