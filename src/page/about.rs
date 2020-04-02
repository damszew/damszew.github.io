use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div![
        attrs!(At::Class => "about_me clearfix container"),
        img![attrs!(At::Src => "https://picsum.photos/450",  At::Class => "box2")],
        div![
            attrs!(At::Class => "box1"),
            h1!["Damian Szewczyk"],
            p![attrs!(At::Class => "social_icons"), "[LinkedIn] [GitHub]"],
            p!["Beginner Rustacean with passion of learning new technologies"],
        ],
    ]
}
