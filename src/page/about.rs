use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div![
        attrs!(At::Class => "content-box about_me"),
        h1!["Damian Szewczyk"],
        div![
            attrs!(At::Class => "social_icons"),
            a![attrs!(At::Href => "https://www.linkedin.com/in/szewczyk-damian", At::Class => "fab fa-linkedin")],
            a![attrs!(At::Href => "https://github.com/damszew", At::Class => "fab fa-github")],
        ],
        p![
            "I'm professional C++ software developer also working with Yocto and CMake.",
            br![],br![],
            "In my free time I like tinkering 🔧, learning new technologies 💻 and gym 💪.
            My recent new favourite is ", em!["Rust"], ", so most of my personal projects are made with it."
        ],
    ]
}
