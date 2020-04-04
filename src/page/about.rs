use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div![
        attrs!(At::Class => "content-box"),
        img![attrs!(At::Src => "https://picsum.photos/450")],
        div![
            h1!["Damian Szewczyk"],
            div![
                attrs!(At::Class => "social_icons"),
                a![attrs!(At::Href => "https://www.linkedin.com/in/szewczyk-damian", At::Class => "fab fa-linkedin")],
                a![attrs!(At::Href => "https://github.com/damszew", At::Class => "fab fa-github")],
            ],
            p!["Beginner Rustacean with passion of learning new technologies"],
        ],
    ]
}
