use crate::{Model, Msg, Page};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> impl View<Msg> {
    let current_page = &model.page;

    vec![
        a![
            attrs![ At::Class => if let Page::Projects = current_page { "active" } else { "" } ],
            simple_ev(Ev::Click, Msg::ChangePage(Page::Projects)),
            "My projects"
        ],
        a![
            attrs![ At::Class => if let Page::AboutMe = current_page { "active" } else { "" } ],
            simple_ev(Ev::Click, Msg::ChangePage(Page::AboutMe)),
            "About Me"
        ],
    ]
}
