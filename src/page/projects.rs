use crate::Msg;
use seed::{prelude::*, *};

const POSTS: &[Post] = &[
    Post{
        title: "First open source contribution",
        image: "static/merged_pr.png",
        description: "Recently I started messing around with [godot-rust](https://github.com/godot-rust/godot-rust)
            and while doing so, I ran into one of these problems that felt like they should be easy to solve, but I couldn't get it to work.
            After some time, I decided to ask [\"How to use Godot's resources in rust bindings?\"](https://github.com/godot-rust/godot-rust/issues/559)
            Perhaps [\"Getting Started book\"](https://godot-rust.github.io/book/getting-started.html) will have a chapter on this topic,
            but for now, the simple example which I have prepared seems to be \"good enough\" solution to show others the basics of this library.


It was my first ever open source contribution. This is just a small example
            but first **merge** to open source project feels amazing.
            I can't wait to contribute to OSS again 🔥.
            ",
        link_label: "Link to PR",
        link: "https://github.com/godot-rust/godot-rust/pull/563",
    },
    Post{
        title: "Personal web page",
        image: "static/personal_page_screenshot.png",
        description: "My very first pet project that I want to show to the world.
            Being more of a system level dev, I have always wanted to create something more visual.
            Tackling unknown domain with newly learned language was quite a challenge.
            But thanks to [rust lang](https://www.rust-lang.org/) community, beginner friendly examples provided by [seed-rs](https://seed-rs.org/)
            and incomparably good explanation of fundamentals from [w3schools](https://www.w3schools.com/), it was nothing less than a pleasant challenge.",
        link_label: "Check it out at gitlab",
        link: "https://gitlab.com/damszew/damszew.gitlab.io",
    },
];

type Href = str;

struct Post<'a> {
    title: &'a str,
    image: &'a Href,
    description: &'a str,
    link_label: &'a str,
    link: &'a Href,
}

impl<'a> Post<'_> {
    fn to_html(&self) -> Node<Msg> {
        article![
            attrs!(At::Class => "content-box"),
            img![attrs!(At::Src => self.image, At::Class => "")],
            div![
                attrs!(At::Class => ""),
                h1![self.title],
                Node::from_markdown(self.description),
                a![attrs![ At::Href => self.link], self.link_label,],
            ]
        ]
    }
}

pub struct Model {
    posts: &'static [Post<'static>],
}

impl Default for Model {
    fn default() -> Self {
        Self { posts: &POSTS }
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    model
        .posts
        .iter()
        .map(|post| post.to_html())
        .collect::<Vec<_>>()
}
