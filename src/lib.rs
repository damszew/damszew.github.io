use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
