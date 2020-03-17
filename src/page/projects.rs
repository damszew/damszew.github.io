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
            image: "https://via.placeholder.com/150".into(),
            description: "Short description".into(),
            link: "some link".into(),
        }
    }

    fn to_html(&self) -> Node<Msg> {
        article![
            img![attrs!(At::Src => self.image)],
            h1![self.title],
            self.description,
            a![self.link]
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
