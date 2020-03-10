use seed::{prelude::*, *};

struct Model {
    sidebar: bool,
    page: Page,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            sidebar: true,
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
    ToggleSidebar,
    ChangePage(Page),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleSidebar => model.sidebar = !model.sidebar,
        Msg::ChangePage(page) => model.page = page,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    vec![
        // Sidebar
        div![
            attrs![ At::Class => "sidebar" ],
            button![
                attrs![At::Class => "menu-icon"],
                simple_ev(Ev::Click, Msg::ToggleSidebar),
                "☰"
            ],
            if model.sidebar {
                div![
                    attrs![ At::Class => "menu" ],
                    ul![
                        li![a![
                            attrs![ At::Href => "#about_me" ],
                            simple_ev(Ev::Click, Msg::ChangePage(Page::AboutMe)),
                            "About Me"
                        ]],
                        li![a![
                            attrs![ At::Href => "#projects" ],
                            simple_ev(Ev::Click, Msg::ChangePage(Page::Projects)),
                            "Pet projects"
                        ]],
                        li![a![
                            attrs![ At::Href => "#contact" ],
                            simple_ev(Ev::Click, Msg::ChangePage(Page::Contact)),
                            "Contact"
                        ]],
                    ]
                ]
            } else {
                div![]
            }
        ],
        // Pages
        div![match model.page {
            Page::AboutMe => "AboutMe",
            Page::Projects => "Projects",
            Page::Contact => "Contact",
        }],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
