use crate::Msg;
use seed::{prelude::*, *};

const POSTS: &[Post] = &[
    Post{
        title: "Personal web page",
        image: "https://picsum.photos/800/450",
        description: "My first rust / web personal project.",
        link_label: "Checkout at github",
        link: "https://github.com/damszew/damszew.github.io",
    },
    Post{
        title: "Project title",
        image: "https://picsum.photos/800/450",
        description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus eget blandit nisi. Nulla pretium feugiat lorem. Fusce enim nibh, auctor in diam quis, facilisis semper nibh. Pellentesque vestibulum turpis ut dui pretium, sit amet finibus mi placerat. Phasellus interdum pharetra ipsum, id euismod dui eleifend quis.",
        link_label: "See more...",
        link: "some link",
    },
    Post{
        title: "Project title",
        image: "https://picsum.photos/800/450",
        description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus eget blandit nisi. Nulla pretium feugiat lorem. Fusce enim nibh, auctor in diam quis, facilisis semper nibh. Pellentesque vestibulum turpis ut dui pretium, sit amet finibus mi placerat. Phasellus interdum pharetra ipsum, id euismod dui eleifend quis.",
        link_label: "See more...",
        link: "some link",
    },
    Post{
        title: "Project title",
        image: "https://picsum.photos/800/450",
        description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus eget blandit nisi. Nulla pretium feugiat lorem. Fusce enim nibh, auctor in diam quis, facilisis semper nibh. Pellentesque vestibulum turpis ut dui pretium, sit amet finibus mi placerat. Phasellus interdum pharetra ipsum, id euismod dui eleifend quis.",
        link_label: "See more...",
        link: "some link",
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
                p![self.description],
                a![
                    attrs![ At::Href => self.link],
                    self.link_label,
                ],
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
