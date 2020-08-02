use crate::Msg;
use seed::{prelude::*, *};

const POSTS: &[Post] = &[
    Post{
        title: "Personal web page",
        image: "/static/personal_page_screenshot.png",
        description: "My very first pet project that I want to show to the world.
            Being more of a system level dev, I have always wanted to create something more visual.
            Tackling unknown domain with newly learned language was quite a challange.
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

pub fn view(model: &Model) -> impl View<Msg> {
    model
        .posts
        .iter()
        .map(|post| post.to_html())
        .collect::<Vec<_>>()
}
