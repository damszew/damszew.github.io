use seed::{prelude::*, *};

mod page;

pub struct Model {
    page: Page,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::Projects,
        }
    }
}

#[derive(Clone)]
pub enum Page {
    AboutMe,
    Projects,
}

#[derive(Clone)]
pub enum Msg {
    ChangePage(Page),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(page) => model.page = page,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        page::partial::sidebar::view(model).els(),
        // Pages
        div![
            attrs![ At::Class => "content" ],
            match model.page {
                Page::Projects => page::projects::view().els(),
                Page::AboutMe => page::about::view().els(),
            }
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
