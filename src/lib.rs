use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod components;
mod pages;
mod route;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
