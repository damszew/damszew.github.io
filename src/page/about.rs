use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div![
        attrs!(At::Class => "about_me"),
        h1!["Damian Szewczyk"],
        img![attrs!(At::Src => "https://via.placeholder.com/150")],
        p!["Beginner Rustacean with passion of learning new technologies"],
    ]
}
