mod app;
mod api_app;
mod components;
mod core;
mod api;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    // Use API-driven app instead of legacy app
    yew::Renderer::<api_app::ApiApp>::new().render();
}
