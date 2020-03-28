use crate::Msg;
use seed::{prelude::*, *};

pub struct Model {
    posts: Vec<Post>,
}

impl Default for Model {
    fn default() -> Self {
        let mut some_posts = Vec::new();

        some_posts.push(Post::new());
        some_posts.push(Post::new());
        some_posts.push(Post::new());
        some_posts.push(Post::new());
        some_posts.push(Post::new());

        Self { posts: some_posts }
    }
}

struct Post {
    title: String,
    image: String, // will be a path
    description: String,
    link: String, // maybe Url?
}

impl Post {
    fn new() -> Self {
        Self {
            title: "Project title".into(),
            image: "https://picsum.photos/800/450".into(),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus eget blandit nisi. Nulla pretium feugiat lorem. Fusce enim nibh, auctor in diam quis, facilisis semper nibh. Pellentesque vestibulum turpis ut dui pretium, sit amet finibus mi placerat. Phasellus interdum pharetra ipsum, id euismod dui eleifend quis.".into(),
            link: "some link".into(),
        }
    }

    fn to_html(&self) -> Node<Msg> {
        article![
            attrs!(At::Class => "clearfix container"),
            img![attrs!(At::Src => self.image, At::Class => "box1")],
            div![
                attrs!(At::Class => "box2"),
                h1![self.title],
                p![self.description],
                a![
                    attrs![ At::Href => self.link, At::Class => "bottomright"],
                    "see more..."
                ]
            ]
        ]
    }
}

pub fn view(model: &Model) -> impl View<Msg> {
    model
        .posts
        .iter()
        .map(|post| post.to_html())
        .collect::<Vec<_>>()
}
