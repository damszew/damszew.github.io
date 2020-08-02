use seed::{prelude::*, *};

mod page;

pub struct Model {
    page: Page,
    posts: page::projects::Model,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::Projects,
            posts: Default::default(),
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

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        div![
            attrs![ At::Id => "sidebar", At::Class => "center-items" ],
            page::partial::sidebar::view(model).into_nodes(),
        ],
        // Pages
        div![
            attrs![ At::Id => "content" ],
            match model.page {
                Page::Projects => page::projects::view(&model.posts).into_nodes(),
                Page::AboutMe => page::about::view().into_nodes(),
            }
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
