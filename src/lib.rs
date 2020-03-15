use seed::{prelude::*, *};

mod page;

struct Model {
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

fn sidebar_view(model: &Model) -> impl View<Msg> {
    let current_page = &model.page;

    div![
        attrs![ At::Class => "sidebar" ],
        div![
            a![
                attrs![ At::Href => "#projects", At::Class => if let Page::Projects = current_page { "active" } else { "" } ],
                simple_ev(Ev::Click, Msg::ChangePage(Page::Projects)),
                "My projects"
            ],
            a![
                attrs![ At::Href => "#about_me", At::Class => if let Page::AboutMe = current_page { "active" } else { "" } ],
                simple_ev(Ev::Click, Msg::ChangePage(Page::AboutMe)),
                "About Me"
            ],
        ]
    ]
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        sidebar_view(model).els(),
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
