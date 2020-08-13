use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod pages;
use pages::Projects;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Projects>::new().mount_to_body();
}
