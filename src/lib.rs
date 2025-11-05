mod app;
mod components;
mod core;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<app::App>::new().render();
}
