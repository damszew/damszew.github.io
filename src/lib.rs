use seed::{prelude::*, *};

struct Model {
    page: Page,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::AboutMe,
        }
    }
}

#[derive(Clone)]
enum Page {
    AboutMe,
    Projects,
    Contact,
}

#[derive(Clone)]
enum Msg {
    ChangePage(Page),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(page) => model.page = page,
    }
}

fn sidebar_view() -> Node<Msg> {
    div![
        attrs![ At::Class => "sidebar" ],
        div![
            a![
                attrs![ At::Href => "#about_me", At::Class => "active" ],
                simple_ev(Ev::Click, Msg::ChangePage(Page::AboutMe)),
                "About Me"
            ],
            a![
                attrs![ At::Href => "#projects" ],
                simple_ev(Ev::Click, Msg::ChangePage(Page::Projects)),
                "Pet projects"
            ],
            a![
                attrs![ At::Href => "#contact" ],
                simple_ev(Ev::Click, Msg::ChangePage(Page::Contact)),
                "Contact"
            ],
        ]
    ]
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        sidebar_view(),
        // Pages
        div![
            attrs![ At::Class => "content" ],
            match model.page {
                Page::AboutMe => "AboutMe",
                Page::Projects => "Projects",
                Page::Contact => "Contact",
            }
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
